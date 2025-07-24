-- Create indexes for items table

-- Index on owner_id for filtering by user
CREATE INDEX IF NOT EXISTS idx_items_owner_id ON items(owner_id);

-- Index on created_at for time-based queries and sorting
CREATE INDEX IF NOT EXISTS idx_items_created_at ON items(created_at);

-- Index on updated_at for recently updated items
CREATE INDEX IF NOT EXISTS idx_items_updated_at ON items(updated_at);

-- Index on completed status for filtering by completion
CREATE INDEX IF NOT EXISTS idx_items_completed ON items(completed);

-- Composite index for owner + completion status (common query pattern)
CREATE INDEX IF NOT EXISTS idx_items_owner_completed ON items(owner_id, completed);

-- Composite index for owner + creation time (for user's items ordered by date)
CREATE INDEX IF NOT EXISTS idx_items_owner_created ON items(owner_id, created_at);

-- Composite index for owner + updated time (for user's recently updated items)
CREATE INDEX IF NOT EXISTS idx_items_owner_updated ON items(owner_id, updated_at);
