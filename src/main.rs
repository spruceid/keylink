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

use jsonwebtoken::crypto;
use jsonwebtoken::Algorithm;
use jsonwebtoken::{DecodingKey, EncodingKey};
use models::NewKey;
use rocket::request::Form;
use rocket::response::Redirect;
use rocket::{get, routes};
use rocket_airlock::Airlock;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::tera::Context;
use rocket_contrib::templates::Template;
use ssi::der::{BitString, Ed25519PrivateKey, Ed25519PublicKey, OctetString, DER};
use ssi::jwk::Params;
use ssi::jwk::JWK;
use std::env;
use user::User;

#[get("/")]
async fn index(user: User, conn: db::KeysDb) -> Template {
    let user_name = user.name;
    let keys = db::get_keys(user_name.clone(), &conn).await;
    let mut context = Context::new();
    context.insert("keys", &keys);
    context.insert("user_name", &user_name);
    Template::render("my_keys", context.into_json())
}

#[get("/", rank = 2)]
fn index_anon() -> Template {
    let context = Context::new();
    Template::render("index", context.into_json())
}

#[derive(FromForm)]
struct NewKeyForm {
    name: String,
}

#[post("/keys", data = "<new_key_form>")]
async fn new_key(new_key_form: Form<NewKeyForm>, user: User, conn: db::KeysDb) -> Redirect {
    let jwk = JWK::generate_ed25519().unwrap();
    match jwk.params {
        Params::OKP(params) => {
            let new_key = NewKey {
                user: user.name,
                name: new_key_form.name.clone(),
                public_key: params.public_key.0,
                private_key: params.private_key.unwrap().0,
            };
            db::insert_key(new_key, &conn).await
        }
        _ => (),
    }
    Redirect::to("/")
}

#[derive(FromForm, Debug)]
struct SignDocForm {
    key: String,
    doc: String,
}

#[post("/sign", data = "<sign_doc_form>")]
async fn sign_doc(sign_doc_form: Form<SignDocForm>, user: User, conn: db::KeysDb) -> String {
    info_!("Request to sign {:?} for {:?}", sign_doc_form, user);
    let key = db::get_key(user.name, sign_doc_form.key.clone(), &conn).await;
    let public_key = BitString(key.public_key);
    let private_key = OctetString(key.private_key);
    let der_key: DER = Ed25519PrivateKey {
        public_key,
        private_key,
    }
    .into();
    let encoding_key = EncodingKey::from_ed_der(&der_key);
    crypto::sign_bytes(
        &sign_doc_form.doc.as_bytes(),
        &encoding_key,
        Algorithm::EdDSA,
    )
    .unwrap()
}

#[derive(FromForm, Debug)]
struct VerifyDocForm {
    key: String,
    doc: String,
    sig: String,
}

#[post("/verify", data = "<verify_doc_form>")]
async fn verify_doc(verify_doc_form: Form<VerifyDocForm>, user: User, conn: db::KeysDb) -> String {
    info_!("Request to verify {:?} for {:?}", verify_doc_form, user);
    let key = db::get_key(user.name, verify_doc_form.key.clone(), &conn).await;
    let decoding_key = DecodingKey::from_ed_der(&key.public_key);
    crypto::verify_bytes(
        &verify_doc_form.sig,
        &verify_doc_form.doc.as_bytes(),
        &decoding_key,
        Algorithm::EdDSA,
    )
    .unwrap()
    .to_string()
}

#[launch]
fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount(
            "/",
            routes![index, index_anon, new_key, sign_doc, verify_doc],
        )
        .mount(
            "/static",
            StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/static")),
        )
        .attach(db::KeysDb::fairing())
        .attach(Template::fairing())
        .attach(Airlock::<hatch::OidcHatch>::fairing())
}
