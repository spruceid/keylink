mod db;
mod hatch;
mod models;
mod schema;
mod user;

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;

use log::info;
use rocket::{get, info_, log_, response::Redirect, routes};
use rocket_airlock::Airlock;
use user::User;

#[get("/")]
async fn index(user: User, conn: db::KeysDb) -> String {
    format!(
        "Hello {}, there are {} keys in the DB.",
        user.name,
        db::count_keys(&conn).await
    )
}

#[get("/", rank = 2)]
fn index_anon() -> Redirect {
    info_!("Anonymous user requested / -> redirecting to /login");
    Redirect::to("/login")
}

#[launch]
fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![index, index_anon])
        .attach(db::KeysDb::fairing())
        .attach(Airlock::<hatch::OidcHatch>::fairing())
}
