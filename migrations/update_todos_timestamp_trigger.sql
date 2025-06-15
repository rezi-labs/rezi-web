-- Create trigger to update updated_at timestamp for items
CREATE TRIGGER IF NOT EXISTS update_items_timestamp
    AFTER UPDATE ON items
    FOR EACH ROW
BEGIN
    UPDATE items SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
END;
