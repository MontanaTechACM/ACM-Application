table! {
    events (event_id) {
        event_id -> Nullable<Integer>,
        coordinator_id -> Nullable<Integer>,
        event_type_id -> Tinyint,
        name -> Varchar,
        additional_info -> Nullable<Text>,
        location -> Varchar,
        event_time -> Timestamp,
    }
}

table! {
    event_types (event_type_id) {
        event_type_id -> Tinyint,
        name -> Varchar,
        description -> Varchar,
    }
}

table! {
    passwords (password_id) {
        password_id -> Nullable<Integer>,
        password -> Varchar,
        verification_code -> Varchar,
    }
}

table! {
    users (user_id) {
        user_id -> Nullable<Integer>,
        password_id -> Nullable<Integer>,
        user_type -> Tinyint,
        first_name -> Varchar,
        last_name -> Varchar,
        email -> Varchar,
    }
}

table! {
    user_types (user_type_id) {
        user_type_id -> Nullable<Tinyint>,
        name -> Varchar,
        description -> Varchar,
    }
}

joinable!(events -> event_types (event_type_id));
joinable!(events -> users (coordinator_id));
joinable!(users -> passwords (password_id));
joinable!(users -> user_types (user_type));

allow_tables_to_appear_in_same_query!(
    events,
    event_types,
    passwords,
    users,
    user_types,
);
