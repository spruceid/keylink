use crate::diesel::insert_into;
use crate::diesel::query_dsl::filter_dsl::FilterDsl;
use crate::diesel::ExpressionMethods;
use crate::diesel::RunQueryDsl;
use crate::schema::keys::dsl::keys;
use crate::schema::keys::name;
use crate::schema::keys::user;
use rocket_contrib::databases::diesel::prelude::SqliteConnection;

use super::models::Key;
use super::models::NewKey;

use std::vec::Vec;

#[database("keys")]
pub struct KeysDb(SqliteConnection);

pub async fn get_keys(user_name: String, conn: &KeysDb) -> Vec<Key> {
    conn.run(|c| keys.filter(user.eq(user_name)).load::<Key>(c))
        .await
        .unwrap()
}

pub async fn get_key(user_name: String, key_name: String, conn: &KeysDb) -> Key {
    conn.run(|c| {
        keys.filter(user.eq(user_name))
            .filter(name.eq(key_name))
            .first::<Key>(c)
    })
    .await
    .unwrap()
}

pub async fn insert_key(new_key: NewKey, conn: &KeysDb) {
    conn.run(move |c| insert_into(keys).values(new_key).execute(c))
        .await
        .unwrap();
}
