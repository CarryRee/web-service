-- Add migration script here
CREATE TABLE users (
    id BIGINT PRIMARY KEY,                    -- 雪花算法生成的 64 位 ID
    username VARCHAR(80) NOT NULL,            -- 用户名
    email VARCHAR(255) NOT NULL UNIQUE,       -- 邮箱，唯一
    password_hash VARCHAR(255) NOT NULL,      -- 存储 bcrypt 哈希（含盐值）
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP, -- 注册时间
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP, -- 更新时间
    is_active BOOLEAN DEFAULT TRUE,           -- 账户是否激活
    role VARCHAR(20) DEFAULT 'user'           -- 角色
);
