-- Create indexes for full-text search performance
-- Adds GIN indexes on to_tsvector columns for faster FTS queries

CREATE INDEX IF NOT EXISTS idx_users_username_tsvector 
  ON users USING gin(to_tsvector('english', username));

CREATE INDEX IF NOT EXISTS idx_posts_content_tsvector 
  ON posts USING gin(to_tsvector('english', title || ' ' || content));

-- Additional useful indexes
CREATE INDEX IF NOT EXISTS idx_posts_user_id ON posts(user_id);
CREATE INDEX IF NOT EXISTS idx_posts_created_at ON posts(created_at DESC);
