-- ====================================================
-- Migration: Create Accounts and Sessions
-- ====================================================

-- Create schema
CREATE SCHEMA IF NOT EXISTS auth;

-- Extensions
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS pgcrypto;

-- Users table
CREATE TABLE auth.accounts
(
    id            UUID PRIMARY KEY   NOT NULL DEFAULT uuid_generate_v4(),
    username      VARCHAR(50) UNIQUE NOT NULL,
    password_hash TEXT               NOT NULL,
    is_admin      BOOLEAN            NOT NULL DEFAULT FALSE,
    created_at    TIMESTAMPTZ                 DEFAULT NOW(),
    updated_at    TIMESTAMPTZ                 DEFAULT NOW()
);

-- Sessions table
CREATE TABLE auth.sessions
(
    id            UUID PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
    account_id    UUID             NOT NULL REFERENCES auth.accounts (id) ON DELETE CASCADE,
    session_token TEXT UNIQUE      NOT NULL,
    last_used_at  TIMESTAMP                 DEFAULT NOW(),
    expires_at    TIMESTAMP                 DEFAULT (NOW() + INTERVAL '30 days')
);
