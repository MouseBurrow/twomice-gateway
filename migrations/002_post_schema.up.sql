-- ====================================================
-- Migration: Create Accounts and Sessions
-- ====================================================

-- Create schema
CREATE SCHEMA IF NOT EXISTS post;

-- Users table
CREATE TABLE post.posts
(
    id         UUID PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
    userid     UUID             NOT NULL REFERENCES auth.accounts (id) ON DELETE CASCADE,
    content    TEXT             NOT NULL,
    image_url  TEXT,
    created_at TIMESTAMP                 DEFAULT NOW()
);

-- Sessions table
CREATE TABLE post.comments
(
    id       UUID PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
    post_id  UUID             NOT NULL REFERENCES post.posts (id) ON DELETE CASCADE,
    user_id  UUID             NOT NULL REFERENCES auth.accounts (id) ON DELETE CASCADE,
    reply_id UUID             NOT NULL REFERENCES post.comments (id) ON DELETE CASCADE,
    content  TEXT             NOT NULL,
    is_reply BOOLEAN          NOT NULL
);
