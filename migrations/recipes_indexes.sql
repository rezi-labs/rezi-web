-- Create indexes for recipes table

-- Index on owner_id for filtering by user
CREATE INDEX IF NOT EXISTS idx_recipes_owner_id ON recipes(owner_id);

-- Index on created_at for time-based queries and sorting
CREATE INDEX IF NOT EXISTS idx_recipes_created_at ON recipes(created_at);

-- Index on updated_at for recently updated recipes
CREATE INDEX IF NOT EXISTS idx_recipes_updated_at ON recipes(updated_at);

-- Index on title for searching by recipe name
CREATE INDEX IF NOT EXISTS idx_recipes_title ON recipes(title);

-- Index on url for finding recipes by source URL
CREATE INDEX IF NOT EXISTS idx_recipes_url ON recipes(url);

-- Composite index for owner + creation time (for user's recipes ordered by date)
CREATE INDEX IF NOT EXISTS idx_recipes_owner_created ON recipes(owner_id, created_at);

-- Composite index for owner + updated time (for user's recently updated recipes)
CREATE INDEX IF NOT EXISTS idx_recipes_owner_updated ON recipes(owner_id, updated_at);

-- Composite index for owner + title (for user's recipes filtered by name)
CREATE INDEX IF NOT EXISTS idx_recipes_owner_title ON recipes(owner_id, title);

-- Full-text search index on content for recipe content search
CREATE INDEX IF NOT EXISTS idx_recipes_content_fts ON recipes(content);

-- Full-text search index on title for recipe title search
CREATE INDEX IF NOT EXISTS idx_recipes_title_fts ON recipes(title);
