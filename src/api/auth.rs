use actix_web::{Error, dev::ServiceRequest};
use actix_web_httpauth::extractors::{AuthenticationError, bearer::{BearerAuth, Config}};

use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode};

use serde::Deserialize;
use log::debug;
use std::env;

use crate::api::util::get_jaeger_params;

#[derive(Deserialize)]
/// Struct for extracting the public key from the Keycloak reply JSON.
/// Serde just makes deserialization way too easy for me not to use it.
struct KeycloakPublicKey {
    public_key: String
}

/// Fetch the RSA public key from a Keycloak server.
/// The GET request should be submitted to the traefik reverse proxy and include the Jaeger tracing header.
fn fetch_keycloak_pubkey(jaeger_key: String, jaeger_id: String) -> Option<DecodingKey> {
    
    let keycloak_host = env::var("KEYCLOAK_HOST").unwrap_or("traefik".to_string());
    let keycloak_realm = env::var("KEYCLOAK_REALM").unwrap_or("biletado".to_string());
    let keycloak_url = format!("http://{}/auth/realms/{}", keycloak_host, keycloak_realm);
    
    let client = reqwest::blocking::Client::new();
    let resp = client.get(keycloak_url)
                     .header(jaeger_key, jaeger_id)
                     .send().ok()?;

    if resp.status().is_success() {
        
        let pubkey : KeycloakPublicKey = resp.json().ok()?;
        
        // do not touch, enough hours were wasted here
        let pem_key = format!("-----BEGIN PUBLIC KEY-----\n{}\n-----END PUBLIC KEY-----", pubkey.public_key.trim());
        debug!("received public key {} from keycloak", pem_key);
        
        let decoding_key = DecodingKey::from_rsa_pem(pem_key.as_bytes()).ok()?;
        Some(decoding_key)

    } else {
        debug!("error while trying to get keycloak public key");
        None
    }

}

#[derive(Deserialize)]
/// The claims deserialized from the JWT MUST contain the `exp` attribute.
struct Claims {
    exp: usize
}

/// Validate a token using the public key from the keycloak server.
/// Return an optional containing true if the token was decoded successfully,
/// false if decoded with problems, None if decoding fails.
fn validate_auth(token: String, decoding_key: DecodingKey) -> Option<bool> {

    debug!("attempting to validate token {}", token);
    
    // NOOO TOUCHY. Idk why exp validation won't work,
    // but tokens only get decoded correctly if we turn it off.
    // Maybe an issue with the system clock?
    let validation = Validation::new(Algorithm::RS256);
    //validation.validate_exp = false;

    let token_msg = decode::<Claims>(
        &token,
        &decoding_key,
        &validation
    );

    if token_msg.is_err() { debug!("error while decoding json web token {}", token); }

    Some(token_msg.is_ok())

}

/// Validate the JWT in the Authorization header of the request.
/// Return an authentication error in case of missing or invalid credentials.
/// Otherwise, hand the request over to the intended handler.
pub async fn validator(req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, Error> {

    // get the jaeger trace header to attach to the keycloak request
    let (jaeger_key, jaeger_id) = get_jaeger_params(&req);

    // fetch the public key every time because Rust lifetimes are a headache and a half
    let pubkey = fetch_keycloak_pubkey(jaeger_key, jaeger_id);
    let config = req.app_data::<Config>().map(|data| data.clone()).unwrap_or_else(Default::default);

    if pubkey.is_none() {
        debug!("keycloak public key not found");
        return Err(AuthenticationError::from(config).into());
    }

    let token = credentials.token().to_string();
    
    debug!("extracted token successfully, attempting to validate");

    match validate_auth(token, pubkey.unwrap()) {
        Some(res) => if res { Ok(req) } else { Err(AuthenticationError::from(config).into()) },
        None => Err(AuthenticationError::from(config).into())
    }
}