# Database Indexes Documentation

This document describes the indexing strategy for the Morgana Web application database.

## Overview

Indexes have been created for all tables to optimize query performance. The indexing strategy focuses on common query patterns including user-based filtering, time-based sorting, and search functionality.

## Tables and Indexes

### Items Table (`items`)

The items table stores task/todo items for users.

**Single Column Indexes:**

- `idx_items_owner_id` - Optimizes filtering by user
- `idx_items_created_at` - Optimizes time-based queries and sorting
- `idx_items_updated_at` - Optimizes queries for recently updated items
- `idx_items_completed` - Optimizes filtering by completion status

**Composite Indexes:**

- `idx_items_owner_completed` - Optimizes common pattern: user's completed/incomplete items
- `idx_items_owner_created` - Optimizes user's items ordered by creation date
- `idx_items_owner_updated` - Optimizes user's items ordered by update date

### Messages Table (`messages`)

The messages table stores chat messages and AI responses.

**Single Column Indexes:**

- `idx_messages_owner_id` - Optimizes filtering by user
- `idx_messages_created_at` - Optimizes time-based queries and sorting
- `idx_messages_is_user` - Optimizes filtering by message type (user vs AI)
- `idx_messages_content_fts` - Enables full-text search on message content
- `idx_messages_ai_response_fts` - Enables full-text search on AI responses

**Composite Indexes:**

- `idx_messages_owner_created` - Optimizes user's messages ordered by date
- `idx_messages_owner_type` - Optimizes filtering user vs AI messages per user

### Recipes Table (`recipes`)

The recipes table stores recipe data with titles, URLs, and content.

**Single Column Indexes:**

- `idx_recipes_owner_id` - Optimizes filtering by user
- `idx_recipes_created_at` - Optimizes time-based queries and sorting
- `idx_recipes_updated_at` - Optimizes queries for recently updated recipes
- `idx_recipes_title` - Optimizes searching by recipe name
- `idx_recipes_url` - Optimizes finding recipes by source URL
- `idx_recipes_content_fts` - Enables full-text search on recipe content
- `idx_recipes_title_fts` - Enables full-text search on recipe titles

**Composite Indexes:**

- `idx_recipes_owner_created` - Optimizes user's recipes ordered by creation date
- `idx_recipes_owner_updated` - Optimizes user's recipes ordered by update date
- `idx_recipes_owner_title` - Optimizes user's recipes filtered by name

## Query Patterns Optimized

### Common Query Patterns:

1. **User-based filtering**: `WHERE owner_id = ?`
2. **Time-based sorting**: `ORDER BY created_at DESC`
3. **User + status filtering**: `WHERE owner_id = ? AND completed = ?`
4. **User + time sorting**: `WHERE owner_id = ? ORDER BY created_at DESC`
5. **Full-text search**: `WHERE content LIKE '%search_term%'`

### Performance Benefits:

- **Faster user data retrieval**: Owner-based indexes eliminate full table scans
- **Efficient time-based queries**: Date indexes support fast sorting and filtering
- **Quick status filtering**: Boolean indexes for completion and message type
- **Search functionality**: Full-text indexes enable content search features
- **Composite query optimization**: Multi-column indexes reduce query execution time

## Migration Files

The indexes are defined in separate migration files:

- `items_indexes.sql` - Indexes for the items table
- `messages_indexes.sql` - Indexes for the messages table
- `recipes_indexes.sql` - Indexes for the recipes table

These migrations are automatically run after the base table migrations in the application startup sequence.

## Maintenance Notes

- All indexes use `IF NOT EXISTS` to prevent errors on re-runs
- Index names follow the pattern: `idx_{table}_{columns}`
- Full-text search indexes use the `_fts` suffix
- Composite indexes list columns in order of selectivity (most selective first)

## Performance Considerations

- Indexes improve read performance but slightly slow down write operations
- The chosen indexes balance common query patterns with storage overhead
- Monitor query performance and adjust indexes based on actual usage patterns
- Consider dropping unused indexes if application requirements change
