-- Create index for better query performance on witch_results timestamp
CREATE INDEX IF NOT EXISTS idx_witch_results_timestamp ON witch_results(timestamp);
