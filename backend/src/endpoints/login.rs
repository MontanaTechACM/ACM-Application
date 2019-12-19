use crate::db::Connection;
use rocket::{self};
use rocket_contrib::json::{Json, JsonValue};
use crate::models::password::Password;
use crate::models::user::User;
use rocket_failure::errors::*;
use regex::Regex;
use jwt::{
    Header,
    Registered,
    Token,
};
use crypto::sha2::Sha256;
use std::env;

const MIN_PASSWORD_LEN: usize = 6;

#[derive(Serialize, Deserialize)]
struct NewUserMedium {
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Serialize, Deserialize)]
struct LoginMedium {
    pub email: String,
    pub password: String,
}

const INVALID_EMAIL_STR: &str = "Email invalid or already in use";

#[post("/create", data = "<new_user_medium>")]
fn create_user(new_user_medium: Json<NewUserMedium>, connection: Connection) -> ApiResult<Json<User>> {
    let medium = new_user_medium.into_inner();
    if validate_email(&medium.email, &connection) {
        bad_request!(INVALID_EMAIL_STR)
    }
    if validate_password(&medium.password) {
        bad_request!(format!("Password length must be at least {}", MIN_PASSWORD_LEN))
    }
    let password = Password {
        password_id: None,
        password: medium.password,
        verification_code: "".to_string(),
    };
    let inserted = Password::create(password, &connection);
    let user = User {
        user_id: None,
        password_id: inserted.password_id,
        user_type: 1,
        first_name: medium.first_name,
        last_name: medium.last_name,
        email: medium.email,
    };
    let inserted = User::create(user, &connection);
    Ok(Json(inserted))
}

const BAD_LOGIN_STR: &str = "Wrong email or password";
const LOGIN_SUCCESSFUL_STR: &str = "Login successful";

#[post("/login", data = "<login_medium>")]
fn attempt_login(login_medium: Json<LoginMedium>, connection: Connection) -> Result<Json<JsonValue>, Status> {
    let medium = login_medium.into_inner();
    if let Some(user) = User::get_user_by_email_and_password(&medium.email, &medium.password, &connection) {
        let header: Header = Default::default();
        let claims = Registered {
            sub: Some(serde_json::to_string(&user).ok().unwrap()),
            ..Default::default()
        };
        let token = Token::new(header, claims);
        let secret_key = env::var("SECRET_KEY").expect("Secret key must be set");

        token.signed(secret_key.as_bytes(), Sha256::new())
            .map(|message| Json(json!({ "success": LOGIN_SUCCESSFUL_STR, "token": message })))
            .map_err(|_| Status::InternalServerError)
    } else {
        Ok(Json(json!({"error":BAD_LOGIN_STR})))
    }
}

fn validate_email(email: &String, connection: &Connection) -> bool {
    let email_regex = Regex::new(r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})").unwrap();
    if email_regex.is_match(&email) {
        if let Some(user) = User::get_user_by_email(email, &connection) {
            return &user.email == email
        }
    }
    false
}

fn validate_password(password: &String) -> bool {
    password.len() <= MIN_PASSWORD_LEN
}

pub fn mount(rocket: rocket::Rocket) -> rocket::Rocket {
    rocket.mount("/auth", routes![create_user, attempt_login])
}

#[cfg(test)]
mod test {
    use super::rocket;
    use super::LoginMedium;
    use super::NewUserMedium;
    use super::User;
    use crate::init_rocket;
    use rocket::local::Client;
    use rocket::http::Status;

    #[test]
    fn test_attempt_login_bad_email_credential() {
        let mut rocket = init_rocket();
        rocket = rocket.mount("/auth", routes![super::attempt_login]);
        let client = Client::new(rocket).unwrap();
        let body_credentials: LoginMedium = LoginMedium {
            email: String::from("dcaron@mtech.edu"),
            password: String::from("mynamejeff"),
        };
        let body_json = serde_json::to_string::<LoginMedium>(&body_credentials).ok().unwrap();
        let mut response = client.post("/auth/login")
            .body(body_json)
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some(format!("{{\"error\":\"{}\"}}", super::BAD_LOGIN_STR).into()));
    }

    #[test]
    fn test_attempt_login_bad_password_credential() {
        let mut rocket = init_rocket();
        rocket = rocket.mount("/auth", routes![super::attempt_login]);
        let client = Client::new(rocket).unwrap();
        let body_credentials: LoginMedium = LoginMedium {
            email: String::from("jbraun@mtech.edu"),
            password: String::from("bad_password"),
        };
        let body_json = serde_json::to_string::<LoginMedium>(&body_credentials).ok().unwrap();
        let mut response = client.post("/auth/login")
            .body(body_json)
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some(format!("{{\"error\":\"{}\"}}", super::BAD_LOGIN_STR).into()));
    }

    #[test]
    fn test_attempt_login_good_credentials() {
        let mut rocket = init_rocket();
        rocket = rocket.mount("/auth", routes![super::attempt_login]);
        let client = Client::new(rocket).unwrap();
        let body_credentials: LoginMedium = LoginMedium {
            email: String::from("jbraun@mtech.edu"),
            password: String::from("mynamejeff"),
        };
        let mut response = client.post("/auth/login")
            .body(serde_json::to_string::<LoginMedium>(&body_credentials).ok().unwrap())
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        let expected_msg = format!("\"success\":\"{}\"", super::LOGIN_SUCCESSFUL_STR);
        let body = response.body_string().unwrap();
        assert!(body.contains(expected_msg.as_str()));
    }

    #[test]
    fn test_create_user_good_values() {
        let mut rocket = init_rocket();
        rocket = rocket.mount("/auth", routes![super::create_user]);
        let client = Client::new(rocket).unwrap();
        let new_user_body: NewUserMedium  = NewUserMedium {
            email: String::from("pcurtiss@mtech.edu"),
            password: String::from("thehandofgod"),
            first_name: String::from("Phil"),
            last_name: String::from("Curtiss"),
        };
        let body_json = serde_json::to_string::<NewUserMedium>(&new_user_body).ok().unwrap();
        let mut response = client.post("/auth/create")
            .body(body_json)
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        let body = response.body_string().unwrap();
        let new_user_response: User = serde_json::from_str::<User>(&body).ok().unwrap();
        assert_eq!(new_user_body.email, new_user_response.email);
        assert_eq!(new_user_body.first_name, new_user_response.first_name);
        assert_eq!(new_user_body.last_name, new_user_response.last_name);
    }
}