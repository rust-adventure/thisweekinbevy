BEGIN;

CREATE TABLE IF NOT EXISTS devlog(
    id VARBINARY(16) NOT NULL,
    title VARCHAR(500) NOT NULL,
    -- url to the video, if exists
    video_url VARCHAR(500) NOT NULL DEFAULT '',
    -- url to the written post, if exists
    post_url VARCHAR(500) NOT NULL DEFAULT '',
    -- url to the discord thread, if it exists
    discord_url VARCHAR(500) NOT NULL DEFAULT '',
    description VARCHAR(5000) NOT NULL DEFAULT '',
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW() ON UPDATE now(),
    PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS issue__devlog(
    issue_id VARBINARY(16) NOT NULL,
    devlog_id VARBINARY(16) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW() ON UPDATE now(),
    PRIMARY KEY (issue_id, devlog_id)
);

CREATE TABLE IF NOT EXISTS devlog__image(
    devlog_id VARBINARY(16) NOT NULL,
    image_id VARBINARY(16) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW() ON UPDATE now(),
    PRIMARY KEY (devlog_id, image_id)
);

COMMIT;