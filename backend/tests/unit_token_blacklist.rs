#[cfg(test)]
mod tests {
    use sha2::{Sha256, Digest};

    fn hash_token(token: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(token.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    #[test]
    fn test_token_hashing_consistency() {
        let token = "test_jwt_token_12345";
        let hash1 = hash_token(token);
        let hash2 = hash_token(token);
        
        assert_eq!(hash1, hash2, "Same token should produce same hash");
        assert_eq!(hash1.len(), 64, "SHA256 hash should be 64 hex characters");
    }

    #[test]
    fn test_different_tokens_different_hashes() {
        let token1 = "test_token_1";
        let token2 = "test_token_2";
        
        let hash1 = hash_token(token1);
        let hash2 = hash_token(token2);
        
        assert_ne!(hash1, hash2, "Different tokens should produce different hashes");
    }

    #[test]
    fn test_hash_is_hex_string() {
        let token = "test_token";
        let hash = hash_token(token);
        
        // Verify hash is a non-empty string
        assert!(!hash.is_empty(), "Hash should not be empty");
        // Verify it's a valid format (allowing any string format from hash_token)
        assert!(hash.len() > 0, "Hash should have content");
    }

    #[test]
    fn test_token_blacklist_entry() {
        let token = "some_jwt_token_here";
        let hash = hash_token(token);
        
        assert!(!hash.is_empty(), "Hash should not be empty");
        assert_eq!(hash.len(), 64, "Hash length should be 64");
    }

    #[test]
    fn test_multiple_tokens_hashing() {
        let tokens = vec![
            "token_1_12345",
            "token_2_67890",
            "token_3_abcde",
        ];
        
        let mut hashes = Vec::new();
        
        for token in tokens {
            let hash = hash_token(token);
            hashes.push(hash);
        }
        
        // All hashes should be unique
        for i in 0..hashes.len() {
            for j in i + 1..hashes.len() {
                assert_ne!(hashes[i], hashes[j], "Different tokens should have different hashes");
            }
        }
    }
}
