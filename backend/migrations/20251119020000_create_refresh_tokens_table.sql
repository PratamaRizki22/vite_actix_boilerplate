-- Create refresh_tokens table for token rotation and reuse detection
CREATE TABLE refresh_tokens (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    token_hash VARCHAR(255) NOT NULL UNIQUE, -- SHA256 hash of refresh token
    token_family VARCHAR(255) NOT NULL, -- Family ID for rotation tracking
    parent_token_hash VARCHAR(255), -- Hash of parent token (for rotation chain)
    is_revoked BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    expires_at TIMESTAMP NOT NULL,
    rotated_at TIMESTAMP, -- When token was rotated
    reuse_detected BOOLEAN DEFAULT FALSE -- Flag for reuse attacks
);

-- Indexes for efficient queries
CREATE INDEX idx_refresh_tokens_user_id ON refresh_tokens(user_id);
CREATE INDEX idx_refresh_tokens_token_hash ON refresh_tokens(token_hash);
CREATE INDEX idx_refresh_tokens_token_family ON refresh_tokens(token_family);
CREATE INDEX idx_refresh_tokens_expires_at ON refresh_tokens(expires_at);
CREATE INDEX idx_refresh_tokens_user_family ON refresh_tokens(user_id, token_family);
