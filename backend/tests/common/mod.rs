use sqlx::PgPool;
use std::env;

pub async fn setup_test_db() -> PgPool {
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://localhost/web3_auth_test".to_string());
    
    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to test database");
    
    pool
}

pub fn get_test_jwt_token() -> String {
    use chrono::Utc;
    use jsonwebtoken::{encode, EncodingKey, Header};
    
    #[derive(serde::Serialize)]
    struct Claims {
        sub: String,
        exp: i64,
    }

    let claims = Claims {
        sub: "1".to_string(),
        exp: (Utc::now().timestamp() + 3600),
    };

    let key = EncodingKey::from_secret(b"test-secret");
    encode(&Header::default(), &claims, &key).expect("Failed to create test token")
}
