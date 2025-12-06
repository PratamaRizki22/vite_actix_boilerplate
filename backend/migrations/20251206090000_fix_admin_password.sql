-- Fix admin user password - replace plain text with bcrypt hash
-- Hash for "admin123" generated with bcrypt cost 12

-- First, backup the old admin if needed
-- DELETE FROM users WHERE id = 1; -- Uncomment if you want to remove old admin

-- Update admin password with proper bcrypt hash
-- Password: admin123 (hashed with bcrypt)
UPDATE users 
SET password = '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewY5kosgKDCZkJTi',
    updated_at = NOW()
WHERE username = 'admin' AND role = 'admin';

-- Note: The hash above is for password "admin123"
-- You can change it after first login
