-- Create composite index for better query performance on witch_results owner_id and timestamp
CREATE INDEX IF NOT EXISTS idx_witch_results_owner_id_timestamp ON witch_results(owner_id, timestamp);
