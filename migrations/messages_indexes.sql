-- Create indexes for messages table

-- Index on owner_id for filtering by user
CREATE INDEX IF NOT EXISTS idx_messages_owner_id ON messages(owner_id);

-- Index on created_at for time-based queries and sorting
CREATE INDEX IF NOT EXISTS idx_messages_created_at ON messages(created_at);

-- Index on is_user for filtering by message type
CREATE INDEX IF NOT EXISTS idx_messages_is_user ON messages(is_user);

-- Composite index for owner + creation time (for user's messages ordered by date)
CREATE INDEX IF NOT EXISTS idx_messages_owner_created ON messages(owner_id, created_at);

-- Composite index for owner + message type (for filtering user vs AI messages)
CREATE INDEX IF NOT EXISTS idx_messages_owner_type ON messages(owner_id, is_user);

-- Full-text search index on content for message search functionality
CREATE INDEX IF NOT EXISTS idx_messages_content_fts ON messages(content);

-- Full-text search index on ai_response for searching AI responses
CREATE INDEX IF NOT EXISTS idx_messages_ai_response_fts ON messages(ai_response);
