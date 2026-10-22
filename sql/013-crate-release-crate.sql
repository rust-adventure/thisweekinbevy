BEGIN;

ALTER TABLE
    crate_release
ADD
    crate_release__crate VARCHAR(50)
AFTER
    discord_url;

COMMIT;