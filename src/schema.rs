table! {
    session_keys (id) {
        id -> Uuid,
        userid -> Uuid,
        key -> Varchar,
        expiry -> Timestamptz,
        extended_at -> Nullable<Timestamptz>,
        created_at -> Timestamptz,
    }
}

table! {
    users (id) {
        id -> Uuid,
        email -> Varchar,
        password_hash -> Varchar,
        password_salt -> Varchar,
        fail_logins -> Nullable<Int4>,
        locked_until -> Nullable<Timestamptz>,
        created_at -> Timestamptz,
    }
}

joinable!(session_keys -> users (userid));

allow_tables_to_appear_in_same_query!(
    session_keys,
    users,
);
