CREATE TABLE IF NOT EXISTS user_info(
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
    INDEX idx_phone (phone),
    INDEX idx_created_at (created_at)
);
