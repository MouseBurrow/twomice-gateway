-- Drop tables first
DROP TABLE IF EXISTS auth.sessions CASCADE;
DROP TABLE IF EXISTS auth.users CASCADE;

-- Drop schema
DROP SCHEMA IF EXISTS auth CASCADE;

-- Drop extensions
DROP EXTENSION IF EXISTS pgcrypto;
DROP EXTENSION IF EXISTS "uuid-ossp";
