use actix_web::{Error, dev::ServiceRequest};
use actix_web_httpauth::extractors::{AuthenticationError, bearer::{BearerAuth, Config}};

use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode};

use serde::Deserialize;
use log::debug;
use std::env;

use crate::api::util::get_jaeger_params;

#[derive(Deserialize)]
struct KeycloakPublicKey {
    public_key: String
}

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
        
        let mut pem_key = "-----BEGIN CERTIFICATE-----\n".to_owned();
        pem_key.push_str(&pubkey.public_key);
        pem_key.push_str("\n-----END CERTIFICATE-----");
        debug!("received public key {} from keycloak", pubkey.public_key);
        
        let decoding_key = DecodingKey::from_rsa_pem(pem_key.as_bytes()).ok()?;
        Some(decoding_key)

    } else {
        debug!("error while trying to get keycloak public key");
        None
    }

}

#[derive(Deserialize)]
struct Claims { }

fn validate_auth(token: String, decoding_key: DecodingKey) -> Option<bool> {

    let token_msg = decode::<Claims>(
        &token,
        &decoding_key,
        &Validation::new(Algorithm::RS256)
    );

    if token_msg.is_err() {
        debug!("error while decoding json web token {}", token);
    }

    Some(token_msg.is_ok())

}

pub async fn validator(req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, Error> {

    let (jaeger_key, jaeger_id) = get_jaeger_params(&req);
    let pubkey = fetch_keycloak_pubkey(jaeger_key, jaeger_id);

    debug!("attempting to validate credentials");

    let config = req.app_data::<Config>().map(|data| data.clone()).unwrap_or_else(Default::default);

    if pubkey.is_none() { return Err(AuthenticationError::from(config).into()); }

    match validate_auth(credentials.token().to_string(), pubkey.unwrap()) {
        Some(res) => if res { Ok(req) } else { Err(AuthenticationError::from(config).into()) },
        None => Err(AuthenticationError::from(config).into())
    }
}