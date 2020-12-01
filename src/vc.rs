use super::doc::get_iscc_id_text;
use super::models::Key;

use ssi::jwk::{Base64urlUInt, OctetParams, Params, JWK};
use ssi::one_or_many::OneOrMany;
use ssi::vc;
use ssi::vc::{
    Contexts, Credential, CredentialSubject, Issuer, LinkedDataProofOptions, ProofPurpose, URI,
};

pub fn issue_vc(key: Key, doc: &str, title: &str) -> Credential {
    let jwk = JWK {
        params: Params::OKP(OctetParams {
            curve: "Ed25519".to_string(),
            public_key: Base64urlUInt(key.public_key),
            private_key: Some(Base64urlUInt(key.private_key)),
        }),
        public_key_use: None,
        key_operations: None,
        algorithm: None,
        key_id: None,
        x509_url: None,
        x509_certificate_chain: None,
        x509_thumbprint_sha1: None,
        x509_thumbprint_sha256: None,
    };

    let issuer = jwk.to_did().unwrap();
    let verification_method = jwk.to_verification_method().unwrap();

    let mut credential = Credential {
        context: Contexts::Many(vec![vc::Context::URI(URI::String(
            "https://www.w3.org/2018/credentials/v1".to_string(),
        ))]),
        id: None,
        type_: OneOrMany::One("VerifiableCredential".to_string()),
        credential_subject: OneOrMany::One(CredentialSubject {
            id: Some(URI::String(format!(
                "did:iscc:{}",
                get_iscc_id_text(title, doc).unwrap()
            ))),
            identifier: None,
            name: None,
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
    };
    let options = LinkedDataProofOptions {
        verification_method: Some(verification_method),
        proof_purpose: Some(ProofPurpose::AssertionMethod),
        created: None,
        challenge: None,
        domain: None,
        checks: None,
    };
    let proof = credential.generate_proof(&jwk, &options).unwrap();
    credential.add_proof(proof);
    credential
}
