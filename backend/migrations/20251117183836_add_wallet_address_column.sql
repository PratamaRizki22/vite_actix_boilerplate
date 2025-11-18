-- Add wallet_address column to users table
ALTER TABLE users ADD COLUMN wallet_address VARCHAR(255) UNIQUE;
