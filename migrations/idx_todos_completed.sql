-- Create index for better query performance on items completed status
CREATE INDEX IF NOT EXISTS idx_items_completed ON items(completed);
