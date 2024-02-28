BEGIN;

CREATE TABLE IF NOT EXISTS issue (
    -- ulid
    id VARBINARY(16) NOT NULL,
    -- 2024-02-05-bevy-0.13-and-more
    slug VARCHAR(200) NOT NULL CHECK (slug <> ''),
    -- ISO 8601
    -- 2024-02-05
    -- year-month-day
    issue_date DATE NOT NULL,
    cloudinary_public_id VARCHAR(20) NOT NULL CHECK (cloudinary_public_id <> ''),
    -- draft, published
    status VARCHAR(50) NOT NULL DEFAULT 'draft',
    -- Bevy 0.13, and more
    display_name VARCHAR(100) NOT NULL DEFAULT '',
    description VARCHAR(5000) NOT NULL DEFAULT '',
    -- PRDUH0JUS_A
    youtube_id VARCHAR(100) NOT NULL DEFAULT '',
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW() ON UPDATE now(),
    PRIMARY KEY(id),
    UNIQUE (slug)
);

CREATE TABLE IF NOT EXISTS github_label(
    id VARBINARY(16) NOT NULL,
    github_id VARCHAR(100) NOT NULL CHECK (github_id <> ''),
    name VARCHAR(100) NOT NULL DEFAULT '',
    url VARCHAR(500) NOT NULL DEFAULT '',
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW() ON UPDATE now(),
    PRIMARY KEY (id),
    UNIQUE (github_id)
);

-- *only* PRs that have been merged at a specific time
CREATE TABLE IF NOT EXISTS merged_pull_request(
    id VARBINARY(16) NOT NULL,
    -- (gh_id) pull_request ids are technically numbers, but 
    -- always used as strings because they are ids
    -- and the ids are shared between issues and prs
    -- so gh_id + 1 doesn't mean "the next pr".
    github_id VARCHAR(10) NOT NULL CHECK (github_id <> ''),
    title VARCHAR(500) NOT NULL DEFAULT '',
    url VARCHAR(500) NOT NULL DEFAULT '',
    -- github usernames are limited to 39 chars
    author VARCHAR(50) NOT NULL DEFAULT '',
    author_url VARCHAR(500) NOT NULL DEFAULT '',
    merged_at_date DATE,
    labels JSON,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW() ON UPDATE now(),
    PRIMARY KEY (id),
    UNIQUE (github_id)
);

CREATE TABLE IF NOT EXISTS issue__merged_pull_request(
    issue_id VARBINARY(16) NOT NULL,
    merged_pull_request_id VARBINARY(16) NOT NULL CHECK (merged_pull_request_id <> ''),
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW() ON UPDATE now(),
    PRIMARY KEY (issue_id, merged_pull_request_id)
);

CREATE TABLE IF NOT EXISTS crate_release(
    id VARBINARY(16) NOT NULL,
    title VARCHAR(500) NOT NULL CHECK (title <> ''),
    url VARCHAR(500) NOT NULL DEFAULT '',
    discord_url VARCHAR(500) NOT NULL DEFAULT '',
    description VARCHAR(5000) NOT NULL DEFAULT '',
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW() ON UPDATE now(),
    -- TODO: images[]
    PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS issue__crate_release(
    issue_id VARBINARY(16) NOT NULL,
    crate_release_id VARBINARY(16) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW() ON UPDATE now(),
    PRIMARY KEY (issue_id, crate_release_id)
);

CREATE TABLE IF NOT EXISTS image(
    id VARBINARY(16) NOT NULL,
    -- cloudinary public ids are 20 characters
    -- and sdks can reconstitute the urls
    -- with transformations from that
    cloudinary_public_id VARCHAR(20) NOT NULL CHECK (cloudinary_public_id <> ''),
    -- alt text
    description VARCHAR(500) NOT NULL DEFAULT '',
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW() ON UPDATE now(),
    PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS crate_release__image(
    crate_release_id VARBINARY(16) NOT NULL,
    image_id VARBINARY(16) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW() ON UPDATE now(),
    PRIMARY KEY (crate_release_id, image_id)
);

CREATE TABLE IF NOT EXISTS showcase(
    id VARBINARY(16) NOT NULL,
    title VARCHAR(500) NOT NULL CHECK (title <> ''),
    -- url to the project page or git repo
    url VARCHAR(500) NOT NULL DEFAULT '',
    -- url to the discord thread, if it exists
    discord_url VARCHAR(500) NOT NULL DEFAULT '',
    description VARCHAR(5000) NOT NULL DEFAULT '',
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW() ON UPDATE now(),
    PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS issue__showcase(
    issue_id VARBINARY(16) NOT NULL,
    showcase_id VARBINARY(16) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW() ON UPDATE now(),
    PRIMARY KEY (issue_id, showcase_id)
);

CREATE TABLE IF NOT EXISTS showcase__image(
    showcase_id VARBINARY(16) NOT NULL,
    image_id VARBINARY(16) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW() ON UPDATE now(),
    PRIMARY KEY (showcase_id, image_id)
);

CREATE TABLE IF NOT EXISTS educational(
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

CREATE TABLE IF NOT EXISTS issue__educational(
    issue_id VARBINARY(16) NOT NULL,
    educational_id VARBINARY(16) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW() ON UPDATE now(),
    PRIMARY KEY (issue_id, educational_id)
);

CREATE TABLE IF NOT EXISTS educational__image(
    educational_id VARBINARY(16) NOT NULL,
    image_id VARBINARY(16) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW() ON UPDATE now(),
    PRIMARY KEY (educational_id, image_id)
);

CREATE TABLE IF NOT EXISTS issue__contributors(
    issue_id VARBINARY(16) NOT NULL,
    contributor VARCHAR(100) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW() ON UPDATE now(),
    PRIMARY KEY (issue_id)
);

CREATE TABLE IF NOT EXISTS new_pull_request(
    id VARBINARY(16) NOT NULL,
    -- (gh_id) pull_request ids are technically numbers, but 
    -- always used as strings because they are ids
    -- and the ids are shared between issues and prs
    -- so gh_id + 1 doesn't mean "the next pr".
    github_id VARCHAR(10) NOT NULL CHECK (github_id <> ''),
    title VARCHAR(500) NOT NULL DEFAULT '',
    url VARCHAR(500) NOT NULL DEFAULT '',
    -- github usernames are limited to 39 chars
    author VARCHAR(50) NOT NULL DEFAULT '',
    author_url VARCHAR(500) NOT NULL DEFAULT '',
    gh_created_at DATE NOT NULL,
    labels JSON,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW() ON UPDATE now(),
    PRIMARY KEY (id),
    UNIQUE (github_id)
);

CREATE TABLE IF NOT EXISTS issue__new_pull_request(
    issue_id VARBINARY(16) NOT NULL,
    pull_request_id VARBINARY(16) NOT NULL CHECK (pull_request_id <> ''),
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW() ON UPDATE now(),
    PRIMARY KEY (issue_id, pull_request_id)
);

CREATE TABLE IF NOT EXISTS new_github_issue(
    id VARBINARY(16) NOT NULL,
    -- (gh_id) github_issue ids are technically numbers, but 
    -- always used as strings because they are ids
    -- and the ids are shared between issues and prs
    -- so gh_id + 1 doesn't mean "the next issue".
    github_id VARCHAR(10) NOT NULL CHECK (github_id <> ''),
    title VARCHAR(500) NOT NULL DEFAULT '',
    url VARCHAR(500) NOT NULL DEFAULT '',
    -- github usernames are limited to 39 chars
    author VARCHAR(50) NOT NULL DEFAULT '',
    author_url VARCHAR(500) NOT NULL DEFAULT '',
    gh_created_at DATE NOT NULL,
    labels JSON,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW() ON UPDATE now(),
    PRIMARY KEY (id),
    UNIQUE (github_id)
);

CREATE TABLE IF NOT EXISTS issue__new_github_issue(
    issue_id VARBINARY(16) NOT NULL,
    github_issue_id VARBINARY(16) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW() ON UPDATE now(),
    PRIMARY KEY (issue_id, github_issue_id)
);

COMMIT;

-- example query to fetch crate_releases and issue_prs
-- for a given issue. multi-cte for sub-queries
-- WITH issue_prs AS (
--     SELECT
--         issue_id,
--         JSON_ARRAYAGG(
--             JSON_OBJECT(
--                 "id",
--                 pull_requests.id,
--                 "gh_id",
--                 pull_requests.gh_id,
--                 "title",
--                 pull_requests.title
--             )
--         ) AS prs
--     FROM
--         issues__pull_requests
--         INNER JOIN pull_requests ON pull_requests.id = issues__pull_requests.pr_id
--     GROUP BY
--         issue_id
-- ),
-- issue_crates as (
--     SELECT
--         issue_id,
--         JSON_ARRAYAGG(
--             JSON_OBJECT(
--                 "id",
--                 crate_releases.id,
--                 "title",
--                 crate_releases.title
--             )
--         ) AS crates
--     FROM
--         issues__crate_releases
--         INNER JOIN crate_releases ON crate_releases.id = issues__crate_releases.pr_id
--     GROUP BY
--         issue_id
-- )
-- SELECT
--     id,
--     issue_prs.prs,
--     issue_crates.crates
-- FROM
--     issues
--     INNER JOIN issue_prs ON issue_prs.issue_id = issues.id
--     INNER JOIN issue_crates ON issue_crates.issue_id = issues.id;