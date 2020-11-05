use super::schema::keys;
use rocket_contrib::databases::diesel::Queryable;
use serde::Serialize;

#[derive(Queryable, PartialEq, Debug, Serialize)]
pub struct Key {
    pub user: String,
    pub name: String,
    pub public_key: Vec<u8>,
    pub private_key: Vec<u8>,
}

#[derive(Insertable, Clone)]
#[table_name = "keys"]
pub struct NewKey {
    pub user: String,
    pub name: String,
    pub public_key: Vec<u8>,
    pub private_key: Vec<u8>,
}
