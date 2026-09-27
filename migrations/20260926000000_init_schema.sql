-- Final schema, keeping only the latest table definitions. Old incremental
-- migrations (wx_open_id add/drop, table rename, second-to-millisecond data
-- conversion) are folded in here or were one-time data fixes that do not apply
-- to a fresh database.
CREATE TABLE IF NOT EXISTS user_info (
    id BIGINT PRIMARY KEY AUTO_INCREMENT,
    nick_name VARCHAR(255) NOT NULL DEFAULT '',
    avatar VARCHAR(500) NOT NULL DEFAULT '',
    signature VARCHAR(500) NOT NULL DEFAULT '',
    age TINYINT UNSIGNED NOT NULL DEFAULT 0,
    phone VARCHAR(20) NOT NULL DEFAULT '',
    salt VARCHAR(32) NOT NULL DEFAULT '',
    password VARCHAR(255) NOT NULL DEFAULT '',
    created_at BIGINT NOT NULL DEFAULT 0,
    updated_at BIGINT NOT NULL DEFAULT 0,
    deleted_at BIGINT NOT NULL DEFAULT 0,
    status TINYINT NOT NULL DEFAULT 0,
    INDEX idx_phone (phone),
    INDEX idx_created_at (created_at)
);

CREATE TABLE IF NOT EXISTS oauth_account (
    id BIGINT PRIMARY KEY AUTO_INCREMENT,
    user_id BIGINT NOT NULL,
    provider VARCHAR(32) NOT NULL,
    provider_app_id VARCHAR(255) NOT NULL DEFAULT '',
    sub_id VARCHAR(255) NOT NULL,
    created_at BIGINT NOT NULL DEFAULT 0,
    updated_at BIGINT NOT NULL DEFAULT 0,
    UNIQUE KEY uk_oauth_identity (provider, provider_app_id, sub_id),
    KEY idx_oauth_user_id (user_id)
);
