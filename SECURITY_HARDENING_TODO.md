# Security Hardening TODO

## 🔴 CRITICAL (Fix Immediately)

### 1. Fix Web3 Signature Verification Bypass
- [ ] Uncomment signature verification code in `src/auth/web3.rs:100-124`
- [ ] Implement proper ECDSA recovery
- [ ] Add tests for signature validation
- [ ] Test with real wallet signatures

### 2. Make JWT_SECRET Mandatory  
- [ ] Change `unwrap_or_else` to `expect` in `src/main.rs:24`
- [ ] Will panic in production if not set (force config)
- [ ] Update documentation

### 3. Move Web3 Challenges to Database
- [ ] Create migration: `create_web3_challenges_table.sql`
- [ ] Add columns: `address`, `challenge`, `expires_at`, `used_at`
- [ ] Add TTL index on `expires_at`
- [ ] Replace `lazy_static` HashMap with DB queries
- [ ] File to modify: `src/auth/web3.rs:13-15`

### 4. Add Rate Limiting to Web3
- [ ] Rate limit `/web3/challenge` (5/hour per address)
- [ ] Rate limit `/web3/verify` (10/hour per address)
- [ ] Use existing `RateLimiter` class

### 5. Tighten CSP Policy
- [ ] Remove `'unsafe-inline'` from CSP
- [ ] Remove `'unsafe-eval'` from CSP
- [ ] File: `src/middleware/security_headers.rs:77`
- [ ] Test frontend compatibility

---

## 🟠 HIGH (Fix This Week)

### 6. Add Input Validation
- [ ] Create `src/utils/validation.rs` with functions:
  - `validate_username(s) -> Result<&str>`
  - `validate_email(s) -> Result<&str>`
  - `validate_password(s) -> Result<&str>`
  - `validate_wallet_address(s) -> Result<&str>`
  - `validate_length(s, min, max) -> Result<&str>`
- [ ] Apply to all endpoints
- [ ] Add tests

### 7. Load CORS from Environment
- [ ] Add `CORS_ALLOWED_ORIGINS` env var (comma-separated)
- [ ] Parse in `src/main.rs:42-48`
- [ ] Default to localhost in dev
- [ ] Error if empty in production

### 8. Add Content-Length Limits
- [ ] Set max payload: 1MB
- [ ] Set max form: 10MB
- [ ] Add to middleware

### 9. Add Rate Limiting Headers
- [ ] Add `RateLimit-Limit` response header
- [ ] Add `RateLimit-Remaining` response header
- [ ] Add `RateLimit-Reset` response header

---

## 🟡 MEDIUM (Fix Before Release)

### 10. Remove Secrets from Git
- [ ] Create `.env.example` with template values
- [ ] Add `.env` to `.gitignore` (if not already)
- [ ] Remove `.env` from git history: `git rm --cached .env`
- [ ] Document in `README.md`

### 11. Add CSRF Protection
- [ ] Create CSRF middleware
- [ ] Generate tokens on form pages
- [ ] Validate on POST/PUT/DELETE
- [ ] Add to all state-changing endpoints

### 12. Add Security Documentation
- [ ] Create `SECURITY.md` with:
  - Security architecture overview
  - How each feature works
  - Common attack scenarios and mitigations
  - Incident reporting procedure

### 13. Add Environment Validation
- [ ] Validate all required env vars on startup
- [ ] Panic with helpful message if missing
- [ ] Log which vars are loaded

### 14. Add Helmet-like Middleware
- [ ] No-Sniff header ✅ (done)
- [ ] X-Frame-Options ✅ (done)
- [ ] HSTS ✅ (done)
- [ ] Remove Server header ✅ (done)
- [ ] Add: `X-Permitted-Cross-Domain-Policies: none`
- [ ] Add: `Cross-Origin-Embedder-Policy: require-corp`

---

## Test Coverage

- [ ] Test Web3 signature verification (valid/invalid)
- [ ] Test Web3 challenge expiry
- [ ] Test rate limiting (at limit/over limit)
- [ ] Test input validation (valid/invalid/edge cases)
- [ ] Test CORS (allowed/denied origins)
- [ ] Test CSP headers
- [ ] Test JWT secret requirement
- [ ] Test refresh token rotation with new Web3

---

## Files to Modify

```
Priority-Critical:
  - src/auth/web3.rs (signature, challenges)
  - src/main.rs (JWT_SECRET, CORS)
  - src/middleware/security_headers.rs (CSP)
  - migrations/ (new web3_challenges table)

Priority-High:
  - src/utils/validation.rs (create new)
  - src/middleware/rate_limiter.rs (add Web3 limits)
  - src/middleware/content_length.rs (create new)

Priority-Medium:
  - .env / .gitignore
  - .env.example (create new)
  - src/middleware/csrf.rs (create new)
  - SECURITY.md (create new)
```

---

## Estimated Timeline

- **Critical fixes**: 2-3 hours
- **High priority**: 2-3 hours  
- **Medium priority**: 1-2 hours
- **Testing & documentation**: 1-2 hours

**Total: 6-10 hours for production-ready security**

---

## Sign-Off Checklist

After all fixes:
- [ ] All 135+ tests passing
- [ ] New security tests added (min 10)
- [ ] Code reviewed by security-minded developer
- [ ] Penetration testing planned
- [ ] Incident response plan documented
- [ ] Security documentation complete
- [ ] Ready for production deployment
