table! {
    keys (user_id, name) {
        user_id -> Varchar,
        name -> Varchar,
        jwk -> Jsonb,
        created_at -> Timestamptz,
    }
}
