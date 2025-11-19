use actix_web::http::header;
use backend::middleware::rate_limiter::RateLimiter;
use actix_web::test::TestRequest;

#[test]
fn test_rate_limiter_allows_within_limit() {
    let req = TestRequest::default()
        .insert_header((header::REMOTE_ADDR, "127.0.0.1"))
        .to_http_request();

    // First 5 requests should be allowed for limit of 5
    for _ in 0..5 {
        let (is_allowed, remaining, _) = RateLimiter::check_limit(&req, "test_key", 5, 3600);
        assert!(is_allowed, "Request should be allowed");
    }
}

#[test]
fn test_rate_limiter_blocks_over_limit() {
    let req = TestRequest::default()
        .insert_header((header::REMOTE_ADDR, "192.168.1.1"))
        .to_http_request();

    // Make 5 requests (the limit)
    for _ in 0..5 {
        let _ = RateLimiter::check_limit(&req, "test_key_2", 5, 3600);
    }

    // 6th request should be blocked
    let (is_allowed, _, _) = RateLimiter::check_limit(&req, "test_key_2", 5, 3600);
    assert!(!is_allowed, "Request should be blocked after limit exceeded");
}

#[test]
fn test_rate_limiter_tracks_remaining() {
    let req = TestRequest::default()
        .insert_header((header::REMOTE_ADDR, "10.0.0.1"))
        .to_http_request();

    let (_, remaining1, _) = RateLimiter::check_limit(&req, "test_key_3", 10, 3600);
    assert_eq!(remaining1, 9, "Should have 9 remaining after first request");

    let (_, remaining2, _) = RateLimiter::check_limit(&req, "test_key_3", 10, 3600);
    assert_eq!(remaining2, 8, "Should have 8 remaining after second request");
}

#[test]
fn test_rate_limiter_different_ips_independent() {
    let req1 = TestRequest::default()
        .insert_header((header::REMOTE_ADDR, "1.1.1.1"))
        .to_http_request();

    let req2 = TestRequest::default()
        .insert_header((header::REMOTE_ADDR, "2.2.2.2"))
        .to_http_request();

    // Both IPs should have independent limits
    let (allowed1, _, _) = RateLimiter::check_limit(&req1, "test_key_4", 1, 3600);
    assert!(allowed1, "First IP should be allowed");

    let (allowed2, _, _) = RateLimiter::check_limit(&req2, "test_key_4", 1, 3600);
    assert!(allowed2, "Second IP should be allowed independently");
}

#[test]
fn test_web3_challenge_rate_limiting() {
    // Simulate 5 Web3 challenge requests from same IP (should succeed)
    let req = TestRequest::default()
        .insert_header((header::REMOTE_ADDR, "172.16.0.1"))
        .to_http_request();

    for i in 0..5 {
        let (is_allowed, _, _) = RateLimiter::check_limit(&req, "web3_challenge", 5, 3600);
        assert!(is_allowed, "Web3 challenge {} should be allowed", i + 1);
    }

    // 6th request should be blocked (exceeds 5/hour limit)
    let (is_allowed, _, reset) = RateLimiter::check_limit(&req, "web3_challenge", 5, 3600);
    assert!(!is_allowed, "6th Web3 challenge should be blocked");
    assert!(reset > 0, "Should have reset time");
}

#[test]
fn test_web3_verify_rate_limiting() {
    // Simulate 10 Web3 verify requests (should succeed)
    let req = TestRequest::default()
        .insert_header((header::REMOTE_ADDR, "172.16.0.2"))
        .to_http_request();

    for i in 0..10 {
        let (is_allowed, _, _) = RateLimiter::check_limit(&req, "web3_verify", 10, 3600);
        assert!(is_allowed, "Web3 verify {} should be allowed", i + 1);
    }

    // 11th request should be blocked (exceeds 10/hour limit)
    let (is_allowed, _, _) = RateLimiter::check_limit(&req, "web3_verify", 10, 3600);
    assert!(!is_allowed, "11th Web3 verify should be blocked");
}
