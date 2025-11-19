-- Create Web3 challenges table with proper TTL management
CREATE TABLE IF NOT EXISTS web3_challenges (
    id SERIAL PRIMARY KEY,
    address VARCHAR(42) NOT NULL,
    challenge VARCHAR(255) NOT NULL UNIQUE,
    expires_at TIMESTAMP NOT NULL,
    used_at TIMESTAMP NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Indexes for efficient queries
CREATE INDEX IF NOT EXISTS idx_web3_challenges_address ON web3_challenges(address);
CREATE INDEX IF NOT EXISTS idx_web3_challenges_challenge ON web3_challenges(challenge);
CREATE INDEX IF NOT EXISTS idx_web3_challenges_expires ON web3_challenges(expires_at);
