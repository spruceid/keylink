use crate::hatch;
use anyhow::{anyhow, Error};
use hatch::OidcHatch;
use rocket::{
    http::Status,
    info_,
    request::{FromRequest, Outcome},
    serde::Serialize,
    Request,
};
use rocket_airlock::{Airlock, Hatch};

#[derive(Debug, PartialEq, Serialize)]
pub(crate) struct User {
    pub(crate) name: Option<String>,
    pub(crate) email: String,
}

#[rocket::async_trait]
impl<'a> FromRequest<'a> for User {
    type Error = Error;

    async fn from_request(request: &'a Request<'_>) -> Outcome<Self, Self::Error> {
        let cookies = request.cookies();
        match cookies.get_private("oidc_access_token") {
            Some(token_cookie) => {
                let hatch = request
                    .guard::<Airlock<OidcHatch>>()
                    .await
                    .expect(&format!(
                        "Hatch '{}' was not installed into the airlock.",
                        OidcHatch::name()
                    ))
                    .hatch;

                if hatch.validate_access_token(token_cookie.value()) {
                    let name = cookies
                        .get_private("preferred_username")
                        .map(|u| u.value().to_string());
                    if let Some(email) = cookies.get_private("email").map(|e| e.value().to_string())
                    {
                        info_!("User '{}' logged in!", &email);
                        Outcome::Success(User { name, email })
                    } else {
                        Outcome::Failure((Status::InternalServerError, anyhow!("No email cookie.")))
                    }
                } else {
                    Outcome::Forward(())
                }
            }
            _ => Outcome::Forward(()),
        }
    }
}
