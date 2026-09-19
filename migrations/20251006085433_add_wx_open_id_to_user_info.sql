-- Add migration script here

-- Add the wx_open_id column to the user_info table
ALTER TABLE user_info ADD COLUMN wx_open_id VARCHAR(100) NOT NULL DEFAULT '';

-- Add an index on the wx_open_id column
CREATE INDEX idx_wx_open_id ON user_info(wx_open_id);
