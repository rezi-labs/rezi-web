-- Create index for better query performance on witch_results owner_id
CREATE INDEX IF NOT EXISTS idx_witch_results_owner_id ON witch_results(owner_id);
