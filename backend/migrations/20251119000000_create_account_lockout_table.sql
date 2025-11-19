CREATE TABLE IF NOT EXISTS account_lockout (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL UNIQUE,
    failed_attempts INTEGER DEFAULT 0,
    locked_until TIMESTAMP NULL,
    last_attempt TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE INDEX idx_account_lockout_user_id ON account_lockout(user_id);
CREATE INDEX idx_account_lockout_locked_until ON account_lockout(locked_until);
