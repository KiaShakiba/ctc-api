// @generated automatically by Diesel CLI.

diesel::table! {
    caesar_attacks (id) {
        id -> Int4,
        user_id -> Int4,
        message -> Text,
        cipher -> Text,
        created_at -> Timestamptz,
        completed_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    caesar_decrypts (id) {
        id -> Int4,
        user_id -> Int4,
        key -> Int4,
        cipher -> Text,
        created_at -> Timestamptz,
        completed_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    caesar_encrypts (id) {
        id -> Int4,
        user_id -> Int4,
        key -> Int4,
        message -> Text,
        created_at -> Timestamptz,
        completed_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    diffie_hellman_exchanges (id) {
        id -> Int4,
        user_id -> Int4,
        g -> Int8,
        n -> Int8,
        sk_server -> Int8,
        created_at -> Timestamptz,
        completed_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Text,
        password_hash -> Text,
    }
}

diesel::joinable!(caesar_attacks -> users (user_id));
diesel::joinable!(caesar_decrypts -> users (user_id));
diesel::joinable!(caesar_encrypts -> users (user_id));
diesel::joinable!(diffie_hellman_exchanges -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    caesar_attacks,
    caesar_decrypts,
    caesar_encrypts,
    diffie_hellman_exchanges,
    users,
);
