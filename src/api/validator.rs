use uuid::Uuid;

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
