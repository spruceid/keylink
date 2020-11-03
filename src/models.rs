use super::schema::keys;
use rocket_contrib::databases::diesel::{Insertable, Queryable};

#[derive(Queryable)]
pub struct Key {
    pub id: i32,
    pub key: String,
}

#[derive(Insertable)]
#[table_name = "keys"]
pub struct NewKey<'a> {
    pub key: &'a str,
}
