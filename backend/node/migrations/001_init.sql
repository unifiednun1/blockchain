-- UnifiedNUN: Initial DB schema

CREATE TABLE sessions (
    id SERIAL PRIMARY KEY,
    session_id VARCHAR(128) UNIQUE NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE mined_nun (
    id SERIAL PRIMARY KEY,
    session_id VARCHAR(128) NOT NULL,
    action VARCHAR(32) NOT NULL,
    amount BIGINT NOT NULL,
    mined_at TIMESTAMP DEFAULT NOW(),
    claimed BOOLEAN DEFAULT FALSE
);

CREATE TABLE claims (
    id SERIAL PRIMARY KEY,
    session_id VARCHAR(128) NOT NULL,
    wallet_address VARCHAR(128) NOT NULL,
    amount BIGINT NOT NULL,
    claimed_at TIMESTAMP DEFAULT NOW()
);
