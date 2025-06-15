-- Create index for better query performance on chat_messages sender
CREATE INDEX IF NOT EXISTS idx_chat_messages_sender ON chat_messages(sender);
