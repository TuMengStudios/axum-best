-- Add migration script here

-- 添加 wx_open_id 字段到 user_info 表
ALTER TABLE user_info ADD COLUMN wx_open_id VARCHAR(100) NOT NULL DEFAULT '';

-- 为 wx_open_id 字段添加索引
CREATE INDEX idx_wx_open_id ON user_info(wx_open_id);
