use anyhow::{anyhow, Error};
use openidconnect::{
    core::{self, CoreIdTokenClaims, CoreProviderMetadata, CoreResponseType},
    reqwest::async_http_client,
    AuthenticationFlow, AuthorizationCode, ClientId, ClientSecret, CsrfToken, IssuerUrl, Nonce,
    OAuth2TokenResponse, RedirectUrl, Scope, TokenResponse,
};
use rocket::{
    debug_,
    figment::{
        self,
        error::{Actual, Kind},
        providers::Env,
        Figment,
    },
    http::{
        ext::IntoOwned,
        uri::{Absolute, Uri},
        Cookie, CookieJar, SameSite, Status,
    },
    info_,
    request::{FromRequest, Outcome},
    response::{Debug, Redirect},
    warn_,
    yansi::Paint,
    Build, Request, Rocket, Route,
};
use rocket_airlock::{Airlock, Communicator, Hatch};
use std::ops::{Deref, DerefMut};

pub struct CoreClient(core::CoreClient);

impl Deref for CoreClient {
    type Target = core::CoreClient;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for CoreClient {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[rocket::async_trait]
impl Communicator for CoreClient {
    async fn from(rocket: &Rocket<Build>) -> Result<Self, Box<dyn std::error::Error>> {
        let config = HatchConfig::from(Figment::from(rocket.figment()))?;
        let issuer_url =
            IssuerUrl::new(config.discover_url.to_string()).expect("Invalid issuer Url");

        let redirect_url =
            RedirectUrl::new(config.redirect_url.to_string()).expect("Invalid redirect Url");

        info_!(
            "Fetching OpenID Connect discover manifest at: {}",
            Paint::new(config.discover_url.to_string()).underline()
        );
        // Fetch OpenID Connect discovery document.
        let provider_metadata =
            CoreProviderMetadata::discover_async(issuer_url, async_http_client).await?;

        info_!("Initializing OpenID Client");
        // Set up the config for the auth process.
        let client = core::CoreClient::from_provider_metadata(
            provider_metadata,
            ClientId::new(config.client_id),
            Some(ClientSecret::new(config.client_secret)),
        )
        .set_redirect_uri(redirect_url);

        Ok(CoreClient(client))
    }
}

#[allow(dead_code)]
pub struct OidcHatch<'h> {
    pub(crate) config: HatchConfig<'h>,
    pub(crate) client: Option<CoreClient>,
}

impl<'h> OidcHatch<'static> {
    pub fn authorize_url(&self) -> (Absolute<'static>, String, String) {
        info_!("Generating authorization Url from Manifest with random token and nonce.");
        // Generate the authorization URL to which we'll redirect the user.
        let (authorize_url, csrf_state, nonce) = self
            .comm()
            .authorize_url(
                AuthenticationFlow::<CoreResponseType>::AuthorizationCode,
                CsrfToken::new_random,
                Nonce::new_random,
            )
            .add_scope(Scope::new("profile".to_string()))
            .add_scope(Scope::new("email".to_string()))
            .url();

        let authorize_url = Absolute::parse(authorize_url.as_ref()).expect("Valid Url");

        debug_!(
            "Generated redirect authorization url: {}",
            Paint::new(format!(
                "{}://{}",
                authorize_url.scheme(),
                authorize_url.authority().expect("Came from a valid Url")
            ))
            .underline()
        );
        (
            authorize_url.into_owned(),
            csrf_state.secret().to_string(),
            nonce.secret().to_string(),
        )
    }

    pub async fn exchange_token(
        &self,
        auth_response: &AuthenticationResponse,
    ) -> Result<ClaimResponse, Error> {
        let token_request = self
            .comm()
            .exchange_code(AuthorizationCode::new(auth_response.code.to_string()));

        let token_response = token_request.request_async(async_http_client).await?;

        let claims = token_response
            .id_token()
            .ok_or_else(|| {
                anyhow!("No ID token found. Authorization Server seems to only speak OAuth2")
            })?
            .claims(
                &self.comm().id_token_verifier(),
                &Nonce::new(auth_response.nonce.to_string()),
            )?;

        Ok(ClaimResponse {
            access_token: token_response.access_token().secret().to_string(),
            claims: claims.to_owned(),
        })
    }

    pub fn validate_access_token(&self, _access_token: &str) -> bool {
        // Normally you would use self.comm() to communicate with the OpenID Provider and
        // validate the token incl. Session Management, as per https://openid.net/specs/openid-connect-session-1_0.html.
        // But that is currently not implemented in openidconnect-rs.
        true
    }
}

#[rocket::async_trait]
impl<'h> Hatch for OidcHatch<'static> {
    type Comm = CoreClient;

    fn comm(&self) -> &CoreClient {
        self.client
            .as_ref()
            .expect("Communicator should have been connected")
    }

    fn connect_comm(&mut self, comm: Self::Comm) {
        self.client = Some(comm);
    }

    fn name() -> &'static str {
        "OpenID Connect"
    }

    fn routes() -> Vec<Route> {
        rocket::routes![login, login_callback]
    }

    async fn from(
        rocket: &Rocket<Build>,
    ) -> Result<OidcHatch<'static>, Box<dyn std::error::Error>> {
        let config = HatchConfig::from(Figment::from(rocket.figment()))?;
        let oidc_hatch = OidcHatch {
            config,
            client: None,
        };

        Ok(oidc_hatch)
    }
}

#[derive(Debug)]
pub struct HatchConfig<'h> {
    address: String,
    port: u16,
    discover_url: Absolute<'h>,
    redirect_url: Absolute<'h>,
    client_id: String,
    client_secret: String,
}

impl<'h> HatchConfig<'h> {
    pub fn from(config: Figment) -> Result<HatchConfig<'h>, Box<dyn std::error::Error>> {
        let airlock_name = OidcHatch::name().replace(" ", "").to_lowercase();
        let key = |name: &str| format!("airlock.{}.{}", airlock_name, name);
        let figment = config.merge(Env::prefixed("ROCKET_"));
        let address = figment.extract_inner::<String>("address")?;
        let port = figment.extract_inner("port")?;
        let discover_url = figment.extract_inner::<String>(&key("discover_url"))?;
        let redirect_url = figment.extract_inner::<String>(&key("redirect_url"))?;
        Ok(HatchConfig {
            discover_url: to_absolute_url(&discover_url, &address, port)?,
            redirect_url: to_absolute_url(&redirect_url, &address, port)?,
            address,
            port,
            client_id: figment.extract_inner(&key("client_id"))?,
            client_secret: figment.extract_inner(&key("client_secret"))?,
        })
    }
}

fn to_absolute_url<'h>(
    url: &str,
    address: &str,
    port: u16,
) -> Result<Absolute<'h>, figment::Error> {
    match Uri::parse_any(url) {
        Ok(Uri::Absolute(absolute)) => Ok(absolute.into_owned()),
        Ok(Uri::Reference(reference)) => Absolute::parse(&format!(
            "{}{}{}",
            reference.scheme().map(|_| "").unwrap_or("http://"),
            reference
                .authority()
                .map(|_| "")
                .unwrap_or(&format!("{}:{}", address, port)),
            &reference
        ))
        .map_err(|e| {
            Kind::InvalidValue(
                Actual::Other(format!("{} - Got: {}", e, &format!("{}", &reference))),
                "Tried Origin".to_string(),
            )
            .into()
        })
        .map(|uri| uri.into_owned()),
        Ok(Uri::Origin(origin)) => {
            Absolute::parse(&format!("http://{}:{}{}", address, port, &origin))
                .map_err(|e| {
                    Kind::InvalidValue(
                        Actual::Other(format!(
                            "{} - Got: {}",
                            e,
                            &format!("{}:{}{}", address, port, &origin)
                        )),
                        "Tried Origin".to_string(),
                    )
                    .into()
                })
                .map(|uri| uri.into_owned())
        }
        Ok(Uri::Authority(authority)) => Absolute::parse(&format!("{}", &authority))
            .map_err(|e| {
                Kind::InvalidValue(
                    Actual::Other(format!("{} - Got: {}", e, &format!("{}", &authority))),
                    "Tried Authority".to_string(),
                )
                .into()
            })
            .map(|uri| uri.into_owned()),
        Ok(Uri::Asterisk(_)) => Err(Kind::InvalidValue(
            Actual::Other(format!("Got: {}", url)),
            "Expected 'Uri' - Asterisk is not a valid redirect Url".to_string(),
        )
        .into()),
        Err(e) => Err(Kind::InvalidValue(
            Actual::Other(format!("{} - Got: {}", e, url)),
            "Expected 'Uri'".to_string(),
        )
        .into()),
    }
}

#[rocket::get("/login", rank = 2)]
pub fn login(airlock: Airlock<OidcHatch<'static>>, cookies: &CookieJar<'_>) -> Redirect {
    let (authorize_url, csrf_state, nonce) = airlock.hatch.authorize_url();
    cookies.add_private(
        Cookie::build("oidc_state", csrf_state)
            .same_site(SameSite::Lax)
            .finish(),
    );
    cookies.add_private(
        Cookie::build("oidc_nonce", nonce)
            .same_site(SameSite::Lax)
            .finish(),
    );

    info_!("Redirecting to {}", Paint::new(&authorize_url).underline());
    Redirect::to(authorize_url)
}

#[rocket::get("/login")]
pub(crate) async fn login_callback(
    airlock: Airlock<OidcHatch<'static>>,
    auth_response: AuthenticationResponse,
    cookies: &CookieJar<'_>,
) -> Result<Redirect, Debug<Error>> {
    debug_!("[login_callback] Returned code: {}", &auth_response.code);

    // Is part of the OpenID Connect Session Management specification: https://openid.net/specs/openid-connect-session-1_0.html
    // TODO: impl session management
    let _ = auth_response.session_state;

    // Use the token to retrieve the user's information.
    let claim_resonse = airlock.hatch.exchange_token(&auth_response).await?;
    info!("{:?}", claim_resonse);

    if let Some(u) = claim_resonse.claims.preferred_username() {
        cookies.add_private(
            Cookie::build("preferred_username", u.to_string())
                .same_site(SameSite::Lax)
                .finish(),
        );
    }
    cookies.add_private(
        Cookie::build(
            "email",
            claim_resonse
                .claims
                .email()
                .ok_or_else(|| anyhow!("No email in claim."))?
                .to_string(),
        )
        .same_site(SameSite::Lax)
        .finish(),
    );
    cookies.add_private(
        Cookie::build("oidc_access_token", claim_resonse.access_token)
            .same_site(SameSite::Lax)
            .finish(),
    );

    Ok(Redirect::to("/"))
}

#[derive(Debug)]
pub struct ClaimResponse {
    access_token: String,
    claims: CoreIdTokenClaims,
}

pub struct AuthenticationResponse {
    code: String,
    nonce: String,
    session_state: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticationResponse {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let code = request.query_value("code").and_then(|code| code.ok());
        let state: Option<String> = request.query_value("state").and_then(|state| state.ok());
        let session_state = request
            .query_value("session_state")
            .and_then(|session_state| session_state.ok());

        let auth_response = match (code, state, session_state) {
            (Some(code), Some(state), Some(session_state)) => {
                let cookies = request.cookies();

                let state_cookie = cookies.get_private("oidc_state");
                match state_cookie {
                    Some(stored_state) if *stored_state.value() == state => {
                        cookies.remove(stored_state.clone());
                    }
                    other => {
                        if other.is_some() {
                            warn_!("The stored state differs from the state returned from the OpenID Provider.");
                        }
                        return Outcome::Failure((Status::BadRequest, ()));
                    }
                }

                let nonce_cookie = cookies.get_private("oidc_nonce");
                let nonce = match nonce_cookie {
                    Some(stored_nonce) => {
                        cookies.remove(stored_nonce.clone());
                        stored_nonce.value().to_string()
                    }
                    _ => {
                        warn_!("No nonce was stored for the current auth flow.");
                        return Outcome::Failure((Status::BadRequest, ()));
                    }
                };

                AuthenticationResponse {
                    code,
                    nonce,
                    session_state,
                }
            }
            _ => {
                info_!("Either 'code', 'state' or 'session_state' was missing on the providers response.");
                return Outcome::Forward(());
            }
        };

        Outcome::Success(auth_response)
    }
}
