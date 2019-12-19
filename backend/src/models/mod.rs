pub mod event_type;
pub mod event;
pub mod user;
pub mod password;
pub mod usertype;

use crypto::sha2::Sha256;
use crate::crypto::digest::Digest;

fn seed_new_password(password: String) -> String {
    let mut seed = Sha256::new();
    seed.input_str(password.as_str());
    seed.result_str()
}