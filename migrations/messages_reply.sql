-- Add reply_to_id column to messages table for reply functionality
ALTER TABLE messages ADD COLUMN reply_to_id INTEGER REFERENCES messages(id);