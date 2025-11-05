-- Create tokens table for authentication token bindings
CREATE TABLE IF NOT EXISTS tokens (
    id VARCHAR(64) PRIMARY KEY,
    user_id VARCHAR(255) NOT NULL,
    expiry TIMESTAMP WITH TIME ZONE NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_tokens_expiry ON tokens(expiry);

-- Create users table for user profiles and settings
CREATE TABLE IF NOT EXISTS users (
    id VARCHAR(255) PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    settings JSONB NOT NULL DEFAULT '{}'::jsonb
);
