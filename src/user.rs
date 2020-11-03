use crate::hatch;
use hatch::OidcHatch;
use log::info;
use rocket::{
    info_, log_,
    request::{FromRequest, Outcome},
    Request,
};
use rocket_airlock::{Airlock, Hatch};

#[derive(Debug)]
pub(crate) struct User {
    pub(crate) name: String,
}

#[rocket::async_trait]
impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = ();

    async fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        let cookies = request.cookies();
        match cookies.get_private("oicd_access_token") {
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
                    let username = cookies.get_private("username").unwrap().value().to_string();

                    info_!("User '{}' logged in!", &username);
                    return Outcome::Success(User { name: username });
                }

                Outcome::Forward(())
            }
            _ => Outcome::Forward(()),
        }
    }
}
