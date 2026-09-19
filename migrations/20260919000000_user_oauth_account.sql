CREATE TABLE user_oauth_account (
    id BIGINT PRIMARY KEY AUTO_INCREMENT,
    user_id BIGINT NOT NULL,
    provider VARCHAR(32) NOT NULL,
    provider_app_id VARCHAR(128) NOT NULL DEFAULT '',
    sub_id VARCHAR(255) NOT NULL,
    created_at BIGINT NOT NULL DEFAULT 0,
    updated_at BIGINT NOT NULL DEFAULT 0,
    UNIQUE KEY uk_oauth_identity (provider, provider_app_id, sub_id),
    KEY idx_oauth_user_id (user_id)
);

INSERT INTO user_oauth_account
    (user_id, provider, provider_app_id, sub_id, created_at, updated_at)
SELECT id, 'wechat', '', wx_open_id, created_at, updated_at
FROM user_info
WHERE wx_open_id <> '';

ALTER TABLE user_info DROP COLUMN wx_open_id;
