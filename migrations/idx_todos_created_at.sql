-- Create index for better query performance on items created_at timestamp
CREATE INDEX IF NOT EXISTS idx_items_created_at ON items(created_at);
