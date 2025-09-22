-- Create chat_messages table for ChatMessage struct
CREATE TABLE IF NOT EXISTS messages (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    owner_id TEXT NOT NULL,
    content TEXT NOT NULL,
    ai_response TEXT NOT NULL,
    is_user BOOLEAN NOT NULL DEFAULT FALSE,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    reply_to_id INTEGER REFERENCES messages(id)
);