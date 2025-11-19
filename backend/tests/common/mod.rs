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

pub async fn create_test_user(pool: &PgPool, username: &str, email: &str, email_verified: bool) -> (i32, String, String) {
    use bcrypt::{hash, DEFAULT_COST};
    
    let password_hash = hash("Test@1234", DEFAULT_COST)
        .expect("Failed to hash password");
    
    let result = sqlx::query!(
        "INSERT INTO users (username, email, password, role, email_verified) 
         VALUES ($1, $2, $3, $4, $5)
         RETURNING id",
        username,
        email,
        password_hash,
        "user",
        email_verified
    )
    .fetch_one(pool)
    .await
    .expect("Failed to create test user");
    
    (result.id, username.to_string(), email.to_string())
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
