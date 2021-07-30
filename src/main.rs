#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;

mod db;
mod doc;
mod hatch;
mod models;
mod schema;
mod user;
mod vc;

use db::KeylinkDbConn;
use did_method_key::DIDKey;
use models::NewKey;
use user::User;

use anyhow::{anyhow, Error};
use rocket::{
    figment::{
        providers::{Env, Format, Toml},
        Figment,
    },
    form::Form,
    fs::{FileServer, TempFile},
    get,
    response::Debug,
    routes,
    serde::json::Json,
    tokio::fs::read,
    Config,
};
use rocket_airlock::Airlock;
use ssi::{
    jwk::{Algorithm, JWK},
    jws,
    vc::{Credential, LinkedDataProofOptions},
};
use std::env;

#[get("/user")]
async fn user_(user: User) -> Json<User> {
    Json(user)
}

// TODO Maybe it should just return empty Vec instead of None
#[get("/keys")]
async fn keys_list(
    user: User,
    conn: KeylinkDbConn,
) -> Result<Option<Json<Vec<String>>>, Debug<Error>> {
    if let Some(keys) = db::get_keys(user.email.clone(), &conn).await? {
        Ok(Some(Json(keys.into_iter().map(|key| key.name).collect())))
    } else {
        Ok(None)
    }
}

#[derive(Deserialize)]
struct NewKeyForm {
    name: String,
}

#[post("/keys", format = "json", data = "<form>")]
async fn keys_create(
    form: Json<NewKeyForm>,
    user: User,
    conn: KeylinkDbConn,
) -> Result<(), Debug<Error>> {
    let jwk = JWK::generate_ed25519().map_err(|e| anyhow!("Key generation errorr: {}", e))?;
    let new_key = NewKey {
        user_id: user.email,
        name: form.name.clone(),
        jwk: serde_json::to_value(jwk)
            .map_err(|e| anyhow!("Error while serializing JWK: {}", e))?,
    };
    db::insert_key(new_key, &conn).await?;
    Ok(())
}

#[derive(FromForm)]
struct BytesSignForm {
    key: String,
    doc: String,
}

#[post("/bytes/sign", data = "<form>")]
async fn bytes_sign(
    form: Form<BytesSignForm>,
    user: User,
    conn: KeylinkDbConn,
) -> Result<Option<Json<String>>, Debug<Error>> {
    if let Some(key) = db::get_key(user.email, form.key.clone(), &conn).await? {
        let sig = jws::sign_bytes_b64(
            Algorithm::EdDSA,
            form.doc.as_bytes(),
            &serde_json::from_value(key.jwk)
                .map_err(|e| anyhow!("Error while deserializing JWK: {}", e))?,
        )
        .map_err(|e| anyhow!("Signing error: {}", e))?;
        Ok(Some(Json(sig)))
    } else {
        Ok(None)
    }
}

#[derive(FromForm)]
struct BytesVerifyForm {
    key: String,
    doc: String,
    sig: String,
}
#[post("/bytes/verify", data = "<form>")]
async fn bytes_verify(
    form: Form<BytesVerifyForm>,
    user: User,
    conn: KeylinkDbConn,
) -> Result<Option<Json<bool>>, Debug<Error>> {
    let decoded_sig = base64::decode_config(form.sig.clone(), base64::URL_SAFE_NO_PAD)
        .map_err(|e| anyhow!("Error decoding signature: {}", e))?;
    if let Some(key) = db::get_key(user.email, form.key.clone(), &conn).await? {
        jws::verify_bytes(
            Algorithm::EdDSA,
            form.doc.as_bytes(),
            &serde_json::from_value(key.jwk)
                .map_err(|e| anyhow!("Error while deserializing JWK: {}", e))?,
            &decoded_sig,
        )
        .map_err(|e| anyhow!("Verification error: {}", e))?;
        // Should probably distinguish hard errors from bad signature.
        Ok(Some(Json(true)))
    } else {
        Ok(None)
    }
}

#[derive(FromForm)]
struct ISCCSignForm<'f> {
    key: String,
    doc: TempFile<'f>,
    options: Option<Json<LinkedDataProofOptions>>,
}

#[post("/iscc/issue", data = "<form>")]
async fn iscc_vc_issue(
    mut form: Form<ISCCSignForm<'_>>,
    user: User,
    conn: KeylinkDbConn,
) -> Result<Option<Json<Credential>>, Debug<Error>> {
    // TODO stream the data directly without writing to disk -- will need changes in the iscc crate
    form.doc
        .persist_to("/tmp/file.txt")
        .await
        .map_err(|e| anyhow!("Could not persist data to file: {}", e))?;
    let file_name = form
        .doc
        .name()
        .ok_or_else(|| anyhow!("File without a name."))?;
    let content_type = form
        .doc
        .content_type()
        .ok_or_else(|| anyhow!("File without a content type."))?;
    let options = if let Some(o) = form.options.as_ref().map(|o| o.0.clone()) {
        o
    } else {
        LinkedDataProofOptions::default()
    };
    if let Some(key) = db::get_key(user.email, form.key.clone(), &conn).await? {
        let credential = vc::issue_iscc_vc(
            key,
            options,
            &read("/tmp/file.txt")
                .await
                .map_err(|e| anyhow!("Could not read file: {}", e))?,
            file_name,
            content_type,
        )
        .await?;
        Ok(Some(Json(credential)))
    } else {
        Ok(None)
    }
}

#[derive(FromForm)]
struct ISCCVerifyForm<'f> {
    credential: Json<Credential>,
    doc: TempFile<'f>,
    options: Option<Json<LinkedDataProofOptions>>,
}

#[post("/iscc/verify", data = "<form>")]
async fn iscc_vc_verify(
    mut form: Form<ISCCVerifyForm<'_>>,
    _user: User,
) -> Result<Json<bool>, Debug<Error>> {
    // TODO stream the data directly without writing to disk -- will need changes in the iscc crate
    form.doc
        .persist_to("/tmp/file.txt")
        .await
        .map_err(|e| anyhow!("Could not persist data to file: {}", e))?;
    let file_name = form
        .doc
        .name()
        .ok_or_else(|| anyhow!("File without a name."))?;
    let content_type = form
        .doc
        .content_type()
        .ok_or_else(|| anyhow!("File without a content type."))?;
    Ok(Json(
        vc::verify_iscc_vc(
            form.credential.0.clone(),
            form.options.as_ref().map(|o| o.0.clone()),
            &read("/tmp/file.txt")
                .await
                .map_err(|e| anyhow!("Could not read file: {}", e))?,
            file_name,
            content_type,
        )
        .await?,
    ))
}

#[derive(Deserialize)]
struct CredentialIssueForm {
    key: String,
    credential: Credential,
    #[serde(default)]
    options: LinkedDataProofOptions,
}
#[post("/credentials/issue", format = "json", data = "<form>")]
async fn vc_issue(
    form: Json<CredentialIssueForm>,
    user: User,
    conn: KeylinkDbConn,
) -> Result<Option<Json<Credential>>, Debug<Error>> {
    if let Some(key) = db::get_key(user.email, form.key.clone(), &conn).await? {
        let jwk = &serde_json::from_value(key.jwk)
            .map_err(|e| anyhow!("Error while deserializing JWK: {}", e))?;
        let mut res = form.credential.clone();
        // TODO consider using DIDKit to cover all cases, e.g. JWT
        let proof = form
            .credential
            .generate_proof(jwk, &form.options)
            .await
            .map_err(|e| anyhow!("Error generating credential proof: {}", e))?;
        res.add_proof(proof);
        Ok(Some(Json(res)))
    } else {
        Ok(None)
    }
}

// TODO add key in the form to make sure the issuer is the correct key?
#[derive(Deserialize)]
struct CredentialVerifyForm {
    credential: Credential,
    #[serde(default)]
    options: LinkedDataProofOptions,
}

#[post("/credentials/verify", format = "json", data = "<form>")]
async fn vc_verify(form: Json<CredentialVerifyForm>, _user: User) -> Json<bool> {
    let res = form
        .credential
        .verify(Some(form.options.clone()), &DIDKey)
        .await;
    Json(res.errors.is_empty())
}

#[launch]
fn rocket() -> _ {
    let config = Figment::from(Config::default())
        // .merge(Serialized::defaults(config::Config::default()))
        .merge(Toml::file("keylink.toml").nested())
        .merge(Env::prefixed("KEYLINK_").split("__").global())
        .merge(Env::prefixed("ROCKET_").global()); // That's just for easy access to ROCKET_LOG_LEVEL
    debug!("{:#?}", config);

    rocket::custom(config)
        .mount(
            "/",
            routes![
                bytes_sign,
                bytes_verify,
                iscc_vc_issue,
                iscc_vc_verify,
                keys_create,
                keys_list,
                user_,
                // Those endpoints are very similar to the VC-HTTP-API, maybe Keylink should extend that in some way instead of duplicating code
                vc_issue,
                vc_verify
            ],
        )
        .mount(
            "/",
            FileServer::from(concat!(env!("CARGO_MANIFEST_DIR"), "/ui/build")),
        )
        .attach(db::KeylinkDbConn::fairing())
        .attach(Airlock::<hatch::OidcHatch>::fairing())
}
