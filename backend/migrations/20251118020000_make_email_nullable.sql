-- Make email column nullable for Web3 users
ALTER TABLE users ALTER COLUMN email DROP NOT NULL;