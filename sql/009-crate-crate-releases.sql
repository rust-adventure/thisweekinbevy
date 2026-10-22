BEGIN;

CREATE TABLE IF NOT EXISTS crate__crate_release(
    crate_id VARBINARY(16) NOT NULL,
    crate_release_id VARBINARY(16) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW() ON UPDATE now(),
    PRIMARY KEY (crate_id, crate_release_id),
    INDEX (crate_release_id)
);

COMMIT;