# Quick Security Fixes Guide

## ✅ QUICK FIX #1: Make JWT_SECRET Required (5 minutes)

**File**: `src/main.rs`

```diff
- let jwt_secret = std::env::var("JWT_SECRET")
-     .unwrap_or_else(|_| "default-secret-key-change-in-production".to_string());

+ let jwt_secret = std::env::var("JWT_SECRET")
+     .expect("❌ CRITICAL: JWT_SECRET environment variable must be set in production!");
```

**Test**: Remove JWT_SECRET env var and try to run
- ✅ Should panic with clear error message
- ✅ Won't start without proper config

---

## ✅ QUICK FIX #2: Tighten CSP Policy (15 minutes)

**File**: `src/middleware/security_headers.rs` (line 77)

```diff
  headers.insert(
      header::HeaderName::from_static("content-security-policy"),
      header::HeaderValue::from_static(
-         "default-src 'self'; script-src 'self' 'unsafe-inline' 'unsafe-eval'; 
-          style-src 'self' 'unsafe-inline'; img-src 'self' data: https:; 
-          font-src 'self' data:; connect-src 'self' https:; frame-ancestors 'none';"
+         "default-src 'self'; script-src 'self'; 
+          style-src 'self' 'nonce-abc123'; img-src 'self' data: https:; 
+          font-src 'self' data:; connect-src 'self' https:; frame-ancestors 'none';"
      ),
  );
```

**What Changed**:
- ❌ Removed `'unsafe-inline'` from script-src
- ❌ Removed `'unsafe-eval'` from script-src  
- ✅ Added nonce support for critical inline styles

**Test**: Check response headers
```bash
curl -i http://localhost:8080/api/health | grep content-security-policy
```

---

## ✅ QUICK FIX #3: Add Web3 Rate Limiting (30 minutes)

**File**: `src/auth/traditional.rs` - Import at top:
```rust
use crate::middleware::rate_limiter::RateLimiter;
```

**Then in `web3_challenge` function** (add after line 16 in web3.rs):
```rust
pub async fn web3_challenge(
    req: HttpRequest,
    challenge_data: web::Json<Web3ChallengeRequest>,
) -> Result<HttpResponse> {
    // Add rate limiting
    let (is_allowed, remaining, reset_seconds) = 
        RateLimiter::check_limit(&req, "web3_challenge", 5, 3600); // 5/hour
    
    if !is_allowed {
        return Ok(HttpResponse::TooManyRequests().json(serde_json::json!({
            "error": "Too many Web3 challenges. Try again later.",
            "retry_after": reset_seconds
        })));
    }
    
    // ... rest of function
}
```

**Do same for `web3_verify` function**:
```rust
let (is_allowed, remaining, reset_seconds) = 
    RateLimiter::check_limit(&req, "web3_verify", 10, 3600); // 10/hour
```

---

## ✅ QUICK FIX #4: Create .env.example (5 minutes)

**New File**: `.env.example`

```env
# Database
DATABASE_URL=postgres://admin:PASSWORD@localhost:5432/wallet_db
POSTGRES_USER=admin
POSTGRES_PASSWORD=change_this_password_in_production
POSTGRES_DB=wallet_db

# API
JWT_SECRET=generate_random_secret_at_least_32_chars
RUST_LOG=info

# CORS
CORS_ALLOWED_ORIGINS=http://localhost:5173,https://app.example.com

# Email (Turbo SMTP)
EMAIL_PROVIDER=turbo
TURBO_SMTP_USERNAME=your_username
TURBO_SMTP_PASSWORD=your_password
TURBO_SMTP_SERVER=pro.turbo-smtp.com
TURBO_SMTP_PORT=465
```

**Update `.gitignore`**:
```bash
echo ".env" >> .gitignore
echo ".env.*.local" >> .gitignore
```

**Remove from Git**:
```bash
git rm --cached .env
git commit -m "Remove .env from version control"
```

---

## ✅ QUICK FIX #5: Enable Web3 Signature Verification (1 hour)

**File**: `src/auth/web3.rs` (lines 100-126)

**Step 1**: Uncomment the signature verification code:

```rust
// CHANGE FROM THIS:
// let message_hash = hash_message(verify_data.challenge.as_bytes());
// let signature = match verify_data.signature.parse::<Signature>() { ... }
// let recovered_address = match signature.recover(message_hash) { ... }
let recovered_address = verify_data.address.clone(); // BYPASS!

// TO THIS:
let message_hash = hash_message(verify_data.challenge.as_bytes());
let signature = match verify_data.signature.parse::<Signature>() {
    Ok(sig) => sig,
    Err(_) => {
        return Ok(HttpResponse::BadRequest().json(Web3VerifyResponse {
            success: false,
            token: None,
            message: "Invalid signature format".to_string(),
        }));
    }
};

let recovered_address = match signature.recover(message_hash) {
    Ok(addr) => format!("{:?}", addr),
    Err(_) => {
        return Ok(HttpResponse::BadRequest().json(Web3VerifyResponse {
            success: false,
            token: None,
            message: "Invalid signature - could not recover address".to_string(),
        }));
    }
};
```

**Step 2**: Make sure `hash_message` function exists:
```rust
fn hash_message(message: &[u8]) -> [u8; 32] {
    use sha2::{Sha256, Digest};
    let mut hasher = Sha256::new();
    hasher.update(message);
    let result = hasher.finalize();
    let mut hash = [0u8; 32];
    hash.copy_from_slice(&result);
    hash
}
```

**Step 3**: Test with real wallet
```bash
curl -X POST http://localhost:8080/api/auth/web3/verify \
  -H "Content-Type: application/json" \
  -d '{
    "address": "0x1234...",
    "challenge": "challenge_string",
    "signature": "0xsignature..."
  }'
```

Expected: ✅ Success if valid, ❌ "Invalid signature" if invalid

---

## ✅ QUICK FIX #6: Move Web3 Challenges to Database (1 hour)

**Step 1**: Create migration file `migrations/20251119030000_create_web3_challenges_table.sql`

```sql
CREATE TABLE web3_challenges (
    id SERIAL PRIMARY KEY,
    address VARCHAR(42) NOT NULL,
    challenge VARCHAR(255) NOT NULL UNIQUE,
    expires_at TIMESTAMP NOT NULL,
    used_at TIMESTAMP NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_web3_challenges_address ON web3_challenges(address);
CREATE INDEX idx_web3_challenges_challenge ON web3_challenges(challenge);
CREATE INDEX idx_web3_challenges_expires ON web3_challenges(expires_at);

-- Cleanup expired challenges
-- (Run with: DELETE FROM web3_challenges WHERE expires_at < NOW())
```

**Step 2**: Apply migration
```bash
sqlx migrate run
```

**Step 3**: Update `src/auth/web3.rs` to use database instead of HashMap

Remove:
```rust
lazy_static! {
    static ref CHALLENGES: Mutex<HashMap<String, (String, u64)>> = Mutex::new(HashMap::new());
}
```

Add database code for challenge storage and retrieval.

---

## Verification Checklist

After each fix, verify:

```bash
# 1. Code compiles
cd backend && cargo check

# 2. Tests still pass
cargo test

# 3. Security test passes
cargo test --test integration_security_headers

# 4. Git clean
git status

# 5. Env vars correct
echo $JWT_SECRET  # Should be set
echo $CORS_ALLOWED_ORIGINS  # Should be set
```

---

## Deployment Checklist

Before deploying to production:

- [ ] All 5 quick fixes applied
- [ ] Cargo test passes (135+ tests)
- [ ] JWT_SECRET set in production environment
- [ ] CORS_ALLOWED_ORIGINS set correctly
- [ ] Database migrations applied
- [ ] .env not in git history
- [ ] Server running on HTTPS only
- [ ] Database backups configured
- [ ] Monitoring enabled

---

## Emergency Rollback

If issues found in production:

```bash
# Stop application
systemctl stop backend

# Rollback to last known good commit
git revert <commit-hash>

# Rebuild and restart
cargo build --release
systemctl start backend
```

---

## Support

For questions or issues during fixes:
1. Check SECURITY_VULNERABILITIES.md for detailed explanations
2. Review SECURITY_HARDENING_TODO.md for full checklist  
3. Run tests to verify each fix

Remember: **Better to take time now than fix breach later! 🔒**
