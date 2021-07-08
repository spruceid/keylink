use crate::models::{Key, NewKey};
use crate::schema::keys::{dsl::keys, name, user_id};

use anyhow::Result;
use rocket_sync_db_pools::{
    database,
    diesel::{
        insert_into, query_dsl::filter_dsl::FilterDsl, result::Error::NotFound, ExpressionMethods,
        PgConnection, RunQueryDsl,
    },
};
use std::vec::Vec;

#[database("keylink_db")]
pub struct KeylinkDbConn(PgConnection);

pub async fn get_keys(user_id_: String, conn: &KeylinkDbConn) -> Result<Option<Vec<Key>>> {
    match conn
        .run(|c| keys.filter(user_id.eq(user_id_)).load::<Key>(c))
        .await
    {
        Ok(keys_) => {
            if keys_.is_empty() {
                Ok(None)
            } else {
                Ok(Some(keys_))
            }
        }
        Err(NotFound) => Ok(None),
        Err(e) => Err(e.into()),
    }
}

pub async fn get_key(
    user_id_: String,
    key_name: String,
    conn: &KeylinkDbConn,
) -> Result<Option<Key>> {
    match conn
        .run(|c| {
            keys.filter(user_id.eq(user_id_))
                .filter(name.eq(key_name))
                .first::<Key>(c)
        })
        .await
    {
        Ok(key) => Ok(Some(key)),
        Err(NotFound) => Ok(None),
        Err(e) => Err(e.into()),
    }
}

pub async fn insert_key(new_key: NewKey, conn: &KeylinkDbConn) -> Result<()> {
    conn.run(move |c| insert_into(keys).values(new_key).execute(c))
        .await?;
    Ok(())
}
