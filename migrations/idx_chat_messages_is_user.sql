-- Create index for better query performance on chat_messages is_user
CREATE INDEX IF NOT EXISTS idx_chat_messages_is_user ON chat_messages(is_user);
