use crate::diesel::query_dsl::select_dsl::SelectDsl;
use crate::diesel::RunQueryDsl;
use crate::schema::keys::dsl::keys;
use crate::schema::keys::id;

use rocket_contrib::databases::diesel::dsl::count;
use rocket_contrib::databases::diesel::prelude::SqliteConnection;

#[database("keys")]
pub struct KeysDb(SqliteConnection);

pub async fn count_keys(conn: &KeysDb) -> i64 {
    conn.run(|c| keys.select(count(id)).first(c)).await.unwrap()
}
