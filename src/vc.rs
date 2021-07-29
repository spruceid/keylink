use super::doc::get_iscc_id_text;
use super::models::Key;

use anyhow::{anyhow, Result};
use did_method_key::DIDKey;
use rocket::http::ContentType;
use ssi::{
    did::{DIDMethod, Source},
    one_or_many::OneOrMany,
    vc::{self, Contexts, Credential, CredentialSubject, Issuer, LinkedDataProofOptions, URI},
};
use std::convert::TryFrom;

pub async fn issue_iscc_vc(
    key: Key,
    options: LinkedDataProofOptions,
    doc: &[u8],
    title: &str,
    content_type: &ContentType,
) -> Result<Credential> {
    let jwk = serde_json::from_value(key.jwk)?;
    let issuer = DIDKey.generate(&Source::Key(&jwk)).unwrap();

    let iscc = if content_type.is_text() {
        get_iscc_id_text(title, std::str::from_utf8(doc)?)
            .map_err(|_| anyhow!("ISCC generation failed"))?
    } else {
        return Err(anyhow!(
            "Unsupported content type: {}",
            content_type.to_string()
        ));
    };
    let mut credential = Credential {
        context: Contexts::Many(vec![vc::Context::URI(URI::String(
            "https://www.w3.org/2018/credentials/v1".to_string(),
        ))]),
        id: None,
        type_: OneOrMany::One("VerifiableCredential".to_string()),
        credential_subject: OneOrMany::One(CredentialSubject {
            id: Some(URI::String(format!("did:iscc:{}", iscc))),
            property_set: None,
        }),
        issuer: Some(Issuer::URI(URI::String(issuer))),
        issuance_date: Some(chrono::Utc::now()),
        proof: None,
        expiration_date: None,
        credential_status: None,
        terms_of_use: None,
        evidence: None,
        credential_schema: None,
        refresh_service: None,
        property_set: None,
    };
    let proof = credential.generate_proof(&jwk, &options).await?;
    credential.add_proof(proof);
    Ok(credential)
}

pub async fn verify_iscc_vc(
    credential: Credential,
    options: Option<LinkedDataProofOptions>,
    doc: &[u8],
    title: &str,
    content_type: &ContentType,
) -> Result<bool> {
    let iscc = if content_type.is_text() {
        get_iscc_id_text(title, std::str::from_utf8(doc)?)
            .map_err(|_| anyhow!("ISCC generation failed"))?
    } else {
        return Err(anyhow!(
            "Unsupported content type: {}",
            content_type.to_string()
        ));
    };

    // TODO should use DIDKit
    let res = credential.verify(options, &DIDKey).await;
    if !res.errors.is_empty() {
        return Ok(false);
    }
    match credential.credential_subject {
        OneOrMany::One(subject) => {
            if let Some(id) = subject.id {
                if id != URI::try_from(iscc)? {
                    return Err(anyhow!("Wrong subject ID"));
                }
            } else {
                return Err(anyhow!("Credential without subject ID"));
            }
        }
        _ => return Err(anyhow!("Multi subjects not supported")),
    };
    Ok(true)
}
