# Backend Tests

Rust-based integration tests untuk semua fitur authentication dan security menggunakan `cargo test` framework.

## Test Structure

- **Unit Tests** (dalam src modules):
  - `src/utils/auth_tests.rs` - Authentication utilities (password hashing, JWT tokens, roles)
  - `src/middleware/rate_limiter_tests.rs` - Rate limiting logic
  - `src/services/token_blacklist_tests.rs` - Token blacklist hashing

- **Integration Tests** (dalam tests folder):
  - `auth_integration.rs` - Authentication flow tests
  - `email_integration.rs` - Email verification dan Turbo SMTP tests
  - `integration_password_reset.rs` - Password reset dengan 1-hour tokens
  - `integration_sessions.rs` - Session management, 24h timeout, multi-device
  - `integration_ratelimit.rs` - Rate limiting (5/15min login, 5/60min email, 3/60min password)
  - `integration_security_headers.rs` - CSP, HSTS, X-XSS-Protection, dll
  - `integration_blacklist.rs` - Token blacklist dan logout security

- **Common Utilities**: Located in `common/`
  - `common/mod.rs` - Helper functions for test setup

## Running Tests

```bash
# Run all tests
cd backend && cargo test

# Run only integration tests
cargo test --test

# Run specific test file
cargo test --test auth_integration
cargo test --test email_integration
cargo test --test integration_password_reset
cargo test --test integration_sessions
cargo test --test integration_ratelimit
cargo test --test integration_security_headers
cargo test --test integration_blacklist

# Run only unit tests
cargo test --lib

# Run specific test
cargo test test_password_hashing_and_verification

# Run with output
cargo test -- --nocapture

# Run tests in release mode
cargo test --release
```

## Test Coverage

✅ **Authentication**
- Login/logout dengan token verification
- Token blacklisting pada logout
- Invalid credentials handling

✅ **Email Verification**
- 6-digit code generation
- Code validation (matching, expiry)
- Turbo SMTP integration
- Email payload validation

✅ **Password Reset**
- Token generation dan validity (1-hour)
- Single-use token enforcement
- One active reset per email
- Password validation requirements
- Session clearing after reset

✅ **Session Management**
- Session creation dan 24-hour timeout
- Device tracking (multi-device support)
- IP logging dan tracking
- Logout endpoints (current/all/others)

✅ **Rate Limiting**
- Login: 5 attempts per 15 minutes
- Email: 5 requests per 60 minutes
- Password Reset: 3 requests per 60 minutes
- Per-IP tracking
- Window reset logic
- HTTP 429 response

✅ **Security Headers**
- Content-Security-Policy (CSP)
- X-Frame-Options (DENY)
- Strict-Transport-Security (HSTS)
- X-XSS-Protection
- X-Content-Type-Options
- Referrer-Policy
- Permissions-Policy
- Server header removal

✅ **Token Blacklisting**
- Blacklist on logout
- SHA256 hashing consistency
- Multiple token blacklisting
- Database persistence
- Expired entry cleanup
- Concurrent access safety

## Dependencies

- Actix-web (HTTP framework)
- SQLx (database)
- Chrono (date/time)
- SHA256 (hashing)
- Serde (JSON serialization)
- curl installed
- jq installed
- PostgreSQL database configured
