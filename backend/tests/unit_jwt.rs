#[cfg(test)]
mod tests {
    use base64::{engine::general_purpose::STANDARD_NO_PAD, Engine};

    #[test]
    fn test_base64_jwt_payload_decoding() {
        // Typical JWT payload (base64url encoded)
        let payload_part = "eyJzdWIiOjEsInVzZXJuYW1lIjoiZm9vYmFyIiwicm9sZSI6InVzZXIiLCJleHAiOjE2OTM4NDk2MDAsImlhdCI6MTY5Mzc2MzIwMH0";
        
        // Decode without padding
        let decoded = STANDARD_NO_PAD.decode(&payload_part);
        assert!(decoded.is_ok(), "Should decode valid base64url");
        
        let payload = decoded.unwrap();
        let payload_str = String::from_utf8(payload).unwrap();
        
        // Verify it contains expected claims
        assert!(payload_str.contains("sub"), "Should contain sub claim");
        assert!(payload_str.contains("username"), "Should contain username claim");
        assert!(payload_str.contains("role"), "Should contain role claim");
        assert!(payload_str.contains("exp"), "Should contain exp claim");
    }

    #[test]
    fn test_jwt_structure() {
        // Real JWT has format: header.payload.signature
        let jwt = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOjEsInVzZXJuYW1lIjoiZm9vYmFyIiwicm9sZSI6InVzZXIiLCJleHAiOjE2OTM4NDk2MDAsImlhdCI6MTY5Mzc2MzIwMH0.some_signature";
        
        let parts: Vec<&str> = jwt.split('.').collect();
        assert_eq!(parts.len(), 3, "JWT should have 3 parts");
        
        // Verify each part is not empty
        for (i, part) in parts.iter().enumerate() {
            assert!(!part.is_empty(), "Part {} should not be empty", i);
        }
    }

    #[test]
    fn test_invalid_jwt_format() {
        let invalid_jwts = vec![
            "only_two_parts",
            "too.many.parts.here.and.more",
            "",
            ".",
        ];
        
        for jwt in invalid_jwts {
            let parts: Vec<&str> = jwt.split('.').collect();
            assert_ne!(parts.len(), 3, "Invalid JWT '{}' should not have exactly 3 parts", jwt);
        }
    }

    #[test]
    fn test_jwt_header_structure() {
        use base64::{engine::general_purpose, Engine};
        
        let header_b64 = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9";
        
        let decoded = general_purpose::STANDARD.decode(header_b64);
        assert!(decoded.is_ok(), "Valid JWT header should decode");
        
        let header_str = String::from_utf8(decoded.unwrap()).unwrap();
        assert!(header_str.contains("typ"), "Header should contain typ");
        assert!(header_str.contains("alg"), "Header should contain alg");
    }
}
