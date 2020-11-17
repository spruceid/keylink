mod db;
mod doc;
mod hatch;
mod models;
mod schema;
mod user;
mod vc;

use models::NewKey;
use user::User;

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;

use jsonwebtoken::crypto;
use jsonwebtoken::Algorithm;
use jsonwebtoken::{DecodingKey, EncodingKey};
use rocket::data::{Data, ToByteUnit};
use rocket::http::ContentType;
use rocket::request::Form;
use rocket::response::NamedFile;
use rocket::response::Redirect;
use rocket::{get, routes};
use rocket_airlock::Airlock;
use rocket_contrib::json::Json;
use rocket_contrib::serve::StaticFiles;
use rocket_multipart_form_data::{
    MultipartFormData, MultipartFormDataError, MultipartFormDataField, MultipartFormDataOptions,
};
use ssi::der::{BitString, Ed25519PrivateKey, OctetString, DER};
use ssi::jwk::{Params, JWK};
use ssi::vc::Credential;
use std::env;
use std::str;

#[get("/keys")]
async fn keys(user: User, conn: db::KeysDb) -> Json<Vec<String>> {
    let user_name = user.name;
    let keys = db::get_keys(user_name.clone(), &conn).await;
    Json(keys.into_iter().map(|key| key.name).collect())
}

#[get("/")]
async fn index(_user: User, _conn: db::KeysDb) -> Option<NamedFile> {
    NamedFile::open("vue/dist/index.html").await.ok()
}

#[get("/", rank = 2)]
async fn index_anon() -> Option<NamedFile> {
    NamedFile::open("vue/dist/login.html").await.ok()
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
async fn sign_doc(sign_doc_form: Form<SignDocForm>, user: User, conn: db::KeysDb) -> Json<String> {
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
    let sig = crypto::sign_bytes(
        &sign_doc_form.doc.as_bytes(),
        &encoding_key,
        Algorithm::EdDSA,
    )
    .unwrap();
    Json(sig)
}

#[derive(FromForm, Debug)]
struct VerifyDocForm {
    key: String,
    doc: String,
    sig: String,
}

#[post("/verify", data = "<verify_doc_form>")]
async fn verify_doc(
    verify_doc_form: Form<VerifyDocForm>,
    user: User,
    conn: db::KeysDb,
) -> Json<bool> {
    info_!("Request to verify {:?} for {:?}", verify_doc_form, user);
    let key = db::get_key(user.name, verify_doc_form.key.clone(), &conn).await;
    let decoding_key = DecodingKey::from_ed_der(&key.public_key);
    let check = crypto::verify_bytes(
        &verify_doc_form.sig,
        &verify_doc_form.doc.as_bytes(),
        &decoding_key,
        Algorithm::EdDSA,
    )
    .unwrap();
    Json(check)
}

#[post("/vc", data = "<data>")]
async fn vc_doc(
    content_type: &ContentType,
    data: Data,
    user: User,
    conn: db::KeysDb,
) -> Json<Credential> {
    let options = MultipartFormDataOptions::with_multipart_form_data_fields(vec![
        MultipartFormDataField::raw("file"),
        MultipartFormDataField::raw("key"),
    ]);
    let mut multipart_form_data =
        match MultipartFormData::parse(content_type, data.open(2.mebibytes()), options).await {
            Ok(multipart_form_data) => multipart_form_data,
            Err(err) => match err {
                MultipartFormDataError::DataTooLargeError(_) => {
                    panic!("The file is too large.");
                }
                MultipartFormDataError::DataTypeError(_) => {
                    panic!("The file is not an image.");
                }
                _ => panic!("{:?}", err),
            },
        };
    let file = multipart_form_data.raw.remove("file");
    let key_name = match multipart_form_data.raw.remove("key") {
        Some(mut key_name) => key_name.remove(0).raw,
        None => panic!("No key name"),
    };

    let (title, doc) = match file {
        Some(mut doc) => {
            let raw = doc.remove(0);

            let content_type = raw.content_type;
            let file_name = raw.file_name.unwrap();
            let data = raw.raw;

            (file_name, data)
        }
        None => panic!("Please input a file."),
    };

    let key = db::get_key(
        user.name,
        str::from_utf8(&key_name).unwrap().to_string(),
        &conn,
    )
    .await;

    let credential = vc::issue_vc(key, &str::from_utf8(&doc).unwrap(), &title);

    Json(credential)
}

#[launch]
fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount(
            "/",
            routes![index, index_anon, new_key, sign_doc, verify_doc, vc_doc, keys,],
        )
        .mount(
            "/",
            StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/vue/dist")),
        )
        .attach(db::KeysDb::fairing())
        .attach(Airlock::<hatch::OidcHatch>::fairing())
}
