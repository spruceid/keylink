use super::schema::keys;
use chrono::NaiveDateTime;
use rocket_sync_db_pools::diesel::Queryable;
use serde::Serialize;

#[derive(Queryable, PartialEq, Debug, Serialize)]
pub struct Key {
    pub user_id: String,
    pub name: String,
    pub jwk: serde_json::Value,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Clone)]
#[table_name = "keys"]
pub struct NewKey {
    pub user_id: String,
    pub name: String,
    pub jwk: serde_json::Value,
}
