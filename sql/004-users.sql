BEGIN;

CREATE TABLE IF NOT EXISTS github_users (
    github_id VARCHAR(50) NOT NULL,
    username VARCHAR(50) NOT NULL,
    access_token TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW() ON UPDATE now(),
    PRIMARY KEY (github_id)
);

ALTER TABLE
    crate_release
ADD
    submitted_by VARCHAR(50)
AFTER
    discord_url;

ALTER TABLE
    showcase
ADD
    submitted_by VARCHAR(50)
AFTER
    discord_url;

ALTER TABLE
    educational
ADD
    submitted_by VARCHAR(50)
AFTER
    discord_url;

ALTER TABLE
    devlog
ADD
    submitted_by VARCHAR(50)
AFTER
    discord_url;

-- this matches tower-sessions-stores at https://github.com/maxcountryman/tower-sessions-stores/blob/51cc75817cb98e875d8cf3975363f746857ad0b2/sqlx-store/src/mysql_store.rs#L66C1-L71C14
-- if the session version is updated, we need to revisit this.
CREATE TABLE IF NOT EXISTS session (
    id CHAR(22) PRIMARY KEY NOT NULL,
    data BLOB NOT NULL,
    expiry_date TIMESTAMP(6) NOT NULL
);

COMMIT;