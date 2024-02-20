BEGIN;

ALTER TABLE
    crate_release
ADD
    posted_date DATE
AFTER
    discord_url;

ALTER TABLE
    showcase
ADD
    posted_date DATE
AFTER
    discord_url;

ALTER TABLE
    educational
ADD
    posted_date DATE
AFTER
    discord_url;

ALTER TABLE
    devlog
ADD
    posted_date DATE
AFTER
    discord_url;

COMMIT;