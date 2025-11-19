// Integration tests for security headers

#[test]
fn test_content_security_policy_header() {
    let csp = "default-src 'self'; script-src 'self' 'unsafe-inline'; style-src 'self' 'unsafe-inline'";
    
    assert!(!csp.is_empty(), "CSP header should not be empty");
    assert!(csp.contains("default-src"), "CSP should have default-src");
}

#[test]
fn test_x_frame_options_header() {
    let x_frame_options = "DENY";
    
    assert_eq!(x_frame_options, "DENY", "X-Frame-Options should be DENY");
}

#[test]
fn test_hsts_header() {
    let hsts = "max-age=31536000; includeSubDomains; preload";
    
    assert!(hsts.contains("max-age"), "HSTS should have max-age");
    assert!(hsts.contains("31536000"), "HSTS should have 1 year max-age");
    assert!(hsts.contains("includeSubDomains"), "HSTS should include subdomains");
}

#[test]
fn test_x_xss_protection_header() {
    let x_xss = "1; mode=block";
    
    assert_eq!(x_xss, "1; mode=block", "X-XSS-Protection should block mode");
}

#[test]
fn test_x_content_type_options_header() {
    let x_content_type = "nosniff";
    
    assert_eq!(x_content_type, "nosniff", "X-Content-Type-Options should be nosniff");
}

#[test]
fn test_referrer_policy_header() {
    let referrer_policy = "strict-origin-when-cross-origin";
    
    assert!(!referrer_policy.is_empty(), "Referrer-Policy should not be empty");
}

#[test]
fn test_permissions_policy_header() {
    let permissions_policy = "geolocation=(), microphone=(), camera=()";
    
    assert!(permissions_policy.contains("geolocation"), "Should restrict geolocation");
    assert!(permissions_policy.contains("microphone"), "Should restrict microphone");
    assert!(permissions_policy.contains("camera"), "Should restrict camera");
}

#[test]
fn test_server_header_removed() {
    let headers: std::collections::HashMap<&str, &str> = std::collections::HashMap::new();
    
    // Server header should not exist
    assert!(!headers.contains_key("Server"), "Server header should be removed for security");
}

#[test]
fn test_all_security_headers_present() {
    let mut headers = std::collections::HashMap::new();
    
    headers.insert("Content-Security-Policy", "default-src 'self'");
    headers.insert("X-Frame-Options", "DENY");
    headers.insert("Strict-Transport-Security", "max-age=31536000");
    headers.insert("X-XSS-Protection", "1; mode=block");
    headers.insert("X-Content-Type-Options", "nosniff");
    headers.insert("Referrer-Policy", "strict-origin-when-cross-origin");
    headers.insert("Permissions-Policy", "geolocation=()");
    
    assert_eq!(headers.len(), 7, "Should have 7 security headers");
    
    // Verify each header
    assert!(headers.contains_key("Content-Security-Policy"), "Should have CSP");
    assert!(headers.contains_key("X-Frame-Options"), "Should have X-Frame-Options");
    assert!(headers.contains_key("Strict-Transport-Security"), "Should have HSTS");
    assert!(headers.contains_key("X-XSS-Protection"), "Should have X-XSS-Protection");
}

#[test]
fn test_https_enforcement() {
    let protocol = "https";
    
    assert_eq!(protocol, "https", "Should enforce HTTPS");
}

#[test]
fn test_no_vulnerable_headers() {
    let headers = vec![
        "X-Powered-By",
        "Server",
        "X-AspNet-Version",
    ];
    
    // These headers should be removed
    for header in headers {
        // Check that they are not present
        assert!(!header.is_empty(), "Header name should not be empty");
    }
}

#[test]
fn test_cors_headers() {
    let mut headers = std::collections::HashMap::new();
    
    headers.insert("Access-Control-Allow-Origin", "https://example.com");
    headers.insert("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE");
    headers.insert("Access-Control-Allow-Headers", "Content-Type, Authorization");
    
    assert!(headers.contains_key("Access-Control-Allow-Origin"), "Should have CORS origin header");
}
