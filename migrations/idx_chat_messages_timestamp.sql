-- Create index for better query performance on chat_messages timestamp
CREATE INDEX IF NOT EXISTS idx_chat_messages_timestamp ON chat_messages(timestamp);
