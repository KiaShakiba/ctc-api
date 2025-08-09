// @generated automatically by Diesel CLI.

diesel::table! {
    caesar_attacks (id) {
        id -> Int4,
        user_id -> Int4,
        key -> Nullable<Int4>,
        message -> Text,
        cipher -> Text,
        created_at -> Timestamptz,
        completed_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    caesar_decryptions (id) {
        id -> Int4,
        user_id -> Int4,
        key -> Int4,
        message -> Nullable<Text>,
        cipher -> Text,
        created_at -> Timestamptz,
        completed_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    caesar_encryptions (id) {
        id -> Int4,
        user_id -> Int4,
        key -> Int4,
        message -> Text,
        cipher -> Nullable<Text>,
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
diesel::joinable!(caesar_decryptions -> users (user_id));
diesel::joinable!(caesar_encryptions -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    caesar_attacks,
    caesar_decryptions,
    caesar_encryptions,
    users,
);
