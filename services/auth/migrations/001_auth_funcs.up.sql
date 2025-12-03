CREATE FUNCTION create_account(
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
    INSERT INTO accounts (username, password_hash)
    VALUES (p_username, p_password_hash)
    RETURNING id INTO new_account_id;

    -- Generate secure session token
    SELECT encode(extensions.gen_random_bytes(32), 'hex') INTO new_session_token;

    -- Create the session that is connected to the user
    INSERT INTO sessions (account_id, session_token)
    VALUES (new_account_id, new_session_token);

    RETURN QUERY SELECT new_account_id, new_session_token;
EXCEPTION
    -- Unique Violation error code is 23505
    WHEN unique_violation THEN
        RAISE EXCEPTION 'Username already exists' USING ERRCODE = '23505';
END;
$$;

CREATE FUNCTION login_with_token(
    p_session_token TEXT
)
    RETURNS UUID
    LANGUAGE plpgsql
AS
$$
DECLARE
    existing_id      UUID;
    token_expiration TIMESTAMP;
BEGIN
    -- Fetch session associated with the token
    SELECT account_id, expires_at
    INTO existing_id, token_expiration
    FROM sessions
    WHERE session_token = p_session_token;

    -- If not found, token does not exist at all
    IF NOT FOUND THEN
        RAISE EXCEPTION 'Invalid session token' USING ERRCODE = 'P1001';
    END IF;

    -- The error will be different if it is expired!
    IF token_expiration <= NOW() THEN
        RAISE EXCEPTION 'Expired session token' USING ERRCODE = 'P1002';
    END IF;

    -- Slide the expiration, extend expiration on use
    UPDATE sessions
    SET last_used_at = NOW(),
        expires_at   = NOW() + INTERVAL '30 days'
    WHERE session_token = p_session_token;

    RETURN existing_id;
END;
$$;

CREATE FUNCTION get_password_hash(p_username TEXT) RETURNS TEXT
    LANGUAGE plpgsql AS
$$
DECLARE
    stored_hash TEXT;
BEGIN
    SELECT password_hash
    INTO stored_hash
    FROM accounts
    WHERE username = p_username;

    IF NOT FOUND THEN
        RAISE EXCEPTION 'Invalid username' USING ERRCODE = 'P2001';
    END IF;

    RETURN stored_hash;
END;
$$;

CREATE FUNCTION create_session(p_account_id UUID)
    RETURNS TEXT
    LANGUAGE plpgsql
AS
$$
DECLARE
    new_token TEXT;
BEGIN
    SELECT encode(extensions.gen_random_bytes(32), 'hex') INTO new_token;

    INSERT INTO sessions (account_id, session_token)
    VALUES (p_account_id, new_token);

    RETURN new_token;
END;
$$;

CREATE FUNCTION logout_session(p_session_token TEXT)
    RETURNS BOOLEAN
    LANGUAGE plpgsql
AS
$$
DECLARE
    deleted_count INT;
BEGIN
    DELETE FROM sessions WHERE session_token = p_session_token RETURNING 1 INTO deleted_count;

    RETURN deleted_count IS NOT NULL;
END;
$$;