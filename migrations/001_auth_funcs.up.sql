-- ====================================================
-- Migration: Create Procedures
-- ====================================================

CREATE FUNCTION auth.create_account(
    p_username TEXT,
    p_password_hash TEXT
)
    RETURNS TABLE
            (
                account_id    UUID,
                session_token TEXT
            )
    LANGUAGE plpgsql
AS
$$
DECLARE
    new_account_id    UUID;
    new_session_token TEXT;
BEGIN
    -- Create the account
    INSERT INTO auth.users (username, password_hash)
    VALUES (p_username, p_password_hash)
    RETURNING id INTO new_account_id;

    -- Generate secure session token
    SELECT encode(gen_random_bytes(32), 'hex') INTO new_session_token;

    -- Create the session that is connected to the user
    INSERT INTO auth.sessions (user_id, session_token)
    VALUES (new_account_id, new_session_token);

    RETURN QUERY SELECT new_account_id, new_session_token;
EXCEPTION
    -- If we try to create a user, that already uses this name we should raise an exception
    WHEN unique_violation THEN
        RAISE EXCEPTION 'Username "%" already exists', p_username
            USING ERRCODE = 'unique_violation';
END;
$$;

CREATE FUNCTION auth.user_login(
    p_username TEXT,
    p_password_hash TEXT
)
    RETURNS TABLE
            (
                account_id    UUID,
                session_token TEXT,
                expires_at    TIMESTAMP
            )
    LANGUAGE plpgsql
AS
$$
DECLARE
    existing_id    UUID;
    new_token      TEXT;
    new_expiration TIMESTAMP;
BEGIN
    -- Username is a unique key in our table so we will either get one result or none
    SELECT id INTO existing_id FROM auth.users WHERE username == p_username;

    -- If the id is not found, the name must not exist in the database
    IF NOT FOUND THEN
        RAISE EXCEPTION 'Username %s does not exist', p_username USING ERRCODE = 'invalid_text_representation';
    end if;

    -- Then we check if the password is same or not for the found id
    IF NOT EXISTS (SELECT 1 FROM auth.users WHERE id == existing_id AND password_hash == p_password_hash) THEN
        RAISE EXCEPTION 'Invalid password' USING ERRCODE = 'invalid_password';
    end if;

    -- Generate secure session token
    SELECT encode(gen_random_bytes(32), 'hex') INTO new_token;

    -- Create new session
    INSERT INTO auth.sessions (user_id, session_token)
    VALUES (existing_id, new_token)
    RETURNING expires_at INTO new_expiration;

    RETURN QUERY SELECT existing_id, new_token, new_expiration;
END;
$$;

CREATE FUNCTION auth.login_with_token(
    p_session_token TEXT
)
    RETURNS TABLE
            (
                user_id    INT,
                expires_at TIMESTAMP
            )
    LANGUAGE plpgsql
AS
$$
DECLARE
    session_record RECORD;
    new_expiration TIMESTAMP;
BEGIN
    SELECT user_id, expires_at
    INTO session_record
    FROM auth.sessions
    WHERE session_token == p_session_token
      AND expires_at > NOW();

    IF NOT FOUND THEN
        RAISE EXCEPTION 'Invalid or expired session token' USING ERRCODE = 'invalid_text_representation';
    end if;

    -- Extend expiration
    new_expiration := NOW() + INTERVAL '30 days';

    -- Update last_used_at for sliding window
    UPDATE auth.sessions
    SET last_used_at = NOW(),
        expires_at   = new_expiration
    WHERE session_token = p_session_token;

    RETURN QUERY SELECT session_record.user_id, new_expiration;
END;
$$