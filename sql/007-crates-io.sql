BEGIN;

CREATE TABLE IF NOT EXISTS crates_io (
    crate_name VARCHAR(64) NOT NULL,
    version_string VARCHAR(255) NOT NULL,
    major SMALLINT NOT NULL DEFAULT 0,
    minor SMALLINT NOT NULL DEFAULT 0,
    patch SMALLINT NOT NULL DEFAULT 0,
    prerelease VARCHAR(64) DEFAULT "",
    -- pubtime can be null
    pubtime TIMESTAMP,
    deps JSON DEFAULT ('[]'),
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW() ON UPDATE now(),
    PRIMARY KEY (crate_name, version_string)
);

COMMIT;