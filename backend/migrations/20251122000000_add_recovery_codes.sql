-- Add recovery codes column for 2FA backup
ALTER TABLE users ADD COLUMN IF NOT EXISTS recovery_codes TEXT[];
