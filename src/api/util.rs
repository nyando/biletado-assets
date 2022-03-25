use actix_web::dev::ServiceRequest;

use log::debug;

use std::env;

use uuid::Uuid;

/// Extract Jaeger tracing header from the received request.
/// The header key is an environment variable with the ID `JAEGER_HEADER`.
pub fn get_jaeger_params(req: &ServiceRequest) -> (String, String) {

    let jaeger_key = env::var("JAEGER_HEADER").unwrap_or("Uber-Trace-Id".to_string());
    debug!("found jaeger key {}", jaeger_key);
    let jaeger_id = req.headers().get(&jaeger_key).unwrap().to_str().unwrap().to_string();
    debug!("found jaeger value {}", jaeger_id);

    (jaeger_key, jaeger_id)
    
}

/// Wraps the `uuid` module's string parse function to return an optional UUID from a string.
/// A very useful function that does very useful things.
pub fn validate_uuid(input: String) -> Option<uuid::Uuid> {
    Uuid::parse_str(&input).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_successful_validation() {
        let test_input = "a4a443c6-0aad-4c1f-a623-e2c2dfc5780c".to_string();
        let uuid = validate_uuid(test_input);
        assert!(uuid.is_some());
    }
    
    #[test]
    fn test_invalid_uuid() {
        let test_input = "invalid-uuid".to_string();
        let uuid = validate_uuid(test_input);
        assert!(uuid.is_none());
    }
}
