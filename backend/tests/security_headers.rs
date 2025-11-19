use actix_web::{test, web, App};
use backend::routes::api::config;
use sqlx::PgPool;

#[actix_web::test]
async fn test_csp_header_present() {
    // Test that Content-Security-Policy header is present
    let pool = create_test_pool().await;
    
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool))
            .configure(config)
    ).await;

    let req = test::TestRequest::get()
        .uri("/api/users")
        .to_request();

    let resp = test::call_service(&app, req).await;
    
    assert!(resp.headers().contains_key("content-security-policy"));
    let csp = resp.headers().get("content-security-policy").unwrap().to_str().unwrap();
    assert!(csp.contains("script-src 'self'"));
    assert!(!csp.contains("unsafe-inline"));
    assert!(!csp.contains("unsafe-eval"));
}

#[actix_web::test]
async fn test_hsts_header_present() {
    // Test that Strict-Transport-Security header enforces HTTPS
    let pool = create_test_pool().await;
    
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool))
            .configure(config)
    ).await;

    let req = test::TestRequest::get()
        .uri("/api/users")
        .to_request();

    let resp = test::call_service(&app, req).await;
    
    assert!(resp.headers().contains_key("strict-transport-security"));
    let hsts = resp.headers().get("strict-transport-security").unwrap().to_str().unwrap();
    assert!(hsts.contains("max-age=31536000"));
    assert!(hsts.contains("includeSubDomains"));
}

#[actix_web::test]
async fn test_x_frame_options_deny() {
    // Test that X-Frame-Options prevents clickjacking
    let pool = create_test_pool().await;
    
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool))
            .configure(config)
    ).await;

    let req = test::TestRequest::get()
        .uri("/api/users")
        .to_request();

    let resp = test::call_service(&app, req).await;
    
    assert!(resp.headers().contains_key("x-frame-options"));
    let x_frame = resp.headers().get("x-frame-options").unwrap().to_str().unwrap();
    assert_eq!(x_frame, "DENY");
}

#[actix_web::test]
async fn test_x_content_type_options() {
    // Test that X-Content-Type-Options prevents MIME sniffing
    let pool = create_test_pool().await;
    
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool))
            .configure(config)
    ).await;

    let req = test::TestRequest::get()
        .uri("/api/users")
        .to_request();

    let resp = test::call_service(&app, req).await;
    
    assert!(resp.headers().contains_key("x-content-type-options"));
    let mime_opt = resp.headers().get("x-content-type-options").unwrap().to_str().unwrap();
    assert_eq!(mime_opt, "nosniff");
}

#[actix_web::test]
async fn test_permissions_policy() {
    // Test that Permissions-Policy restricts dangerous APIs
    let pool = create_test_pool().await;
    
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool))
            .configure(config)
    ).await;

    let req = test::TestRequest::get()
        .uri("/api/users")
        .to_request();

    let resp = test::call_service(&app, req).await;
    
    assert!(resp.headers().contains_key("permissions-policy"));
    let perms = resp.headers().get("permissions-policy").unwrap().to_str().unwrap();
    assert!(perms.contains("geolocation=()"));
    assert!(perms.contains("microphone=()"));
    assert!(perms.contains("camera=()"));
}

#[actix_web::test]
async fn test_server_header_removed() {
    // Test that Server header is removed to avoid information disclosure
    let pool = create_test_pool().await;
    
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool))
            .configure(config)
    ).await;

    let req = test::TestRequest::get()
        .uri("/api/users")
        .to_request();

    let resp = test::call_service(&app, req).await;
    
    assert!(!resp.headers().contains_key("server"));
}

// Helper function to create a test database pool
async fn create_test_pool() -> web::Data<PgPool> {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:password@localhost/test".to_string());
    
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .unwrap_or_else(|_| {
            // Return a mock pool if database is not available for testing
            panic!("Could not connect to test database");
        });
    
    web::Data::new(pool)
}
