BEGIN;

CREATE TABLE IF NOT EXISTS link (
    link_id VARBINARY(16) NOT NULL,
    href VARCHAR(500) NOT NULL,
    display_name VARCHAR(50),
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW() ON UPDATE now(),
    PRIMARY KEY (link_id)
);

CREATE TABLE IF NOT EXISTS devlog__link(
    devlog_id VARBINARY(16) NOT NULL,
    link_id VARBINARY(16) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW() ON UPDATE now(),
    PRIMARY KEY (devlog_id, link_id)
);

CREATE TABLE IF NOT EXISTS crate_release__link(
    crate_release_id VARBINARY(16) NOT NULL,
    link_id VARBINARY(16) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW() ON UPDATE now(),
    PRIMARY KEY (crate_release_id, link_id)
);

CREATE TABLE IF NOT EXISTS showcase__link(
    showcase_id VARBINARY(16) NOT NULL,
    link_id VARBINARY(16) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW() ON UPDATE now(),
    PRIMARY KEY (showcase_id, link_id)
);

CREATE TABLE IF NOT EXISTS educational__link(
    educational_id VARBINARY(16) NOT NULL,
    link_id VARBINARY(16) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW() ON UPDATE now(),
    PRIMARY KEY (educational_id, link_id)
);

-- it is useful to show certain crates in certain places in the
-- admin ui. ex: proc macro crates are *usually* not useful to 
-- show when they support the crate a user actually uses.
CREATE TABLE IF NOT EXISTS crates_io_filters (
    crate_name VARCHAR(64) NOT NULL,
    exclude BOOLEAN DEFAULT 0 NOT NULL,
    notes TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW() ON UPDATE now(),
    PRIMARY KEY (crate_name)
);

CREATE TABLE IF NOT EXISTS crate_release__crates_io (
    crate_release_id VARBINARY(16) NOT NULL,
    crate_name VARCHAR(64) NOT NULL,
    version_string VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW() ON UPDATE now(),
    PRIMARY KEY (crate_release_id, crate_name, version_string)
);

--
-- PROJECT
--
CREATE TABLE IF NOT EXISTS project (
    project_id VARBINARY(16) NOT NULL,
    display_name VARCHAR(100) NOT NULL,
    description TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW() ON UPDATE now(),
    PRIMARY KEY (project_id)
);

CREATE TABLE IF NOT EXISTS project__link(
    project_id VARBINARY(16) NOT NULL,
    link_id VARBINARY(16) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW() ON UPDATE now(),
    PRIMARY KEY (project_id, link_id)
);

CREATE TABLE IF NOT EXISTS project__showcase(
    project_id VARBINARY(16) NOT NULL,
    showcase_id VARBINARY(16) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW() ON UPDATE now(),
    PRIMARY KEY (project_id, showcase_id),
    INDEX (showcase_id)
);

CREATE TABLE IF NOT EXISTS project__crate_release(
    project_id VARBINARY(16) NOT NULL,
    crate_release_id VARBINARY(16) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW() ON UPDATE now(),
    PRIMARY KEY (project_id, crate_release_id),
    INDEX (crate_release_id)
);

CREATE TABLE IF NOT EXISTS project__devlog(
    project_id VARBINARY(16) NOT NULL,
    devlog_id VARBINARY(16) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW() ON UPDATE now(),
    PRIMARY KEY (project_id, devlog_id),
    INDEX (devlog_id)
);

CREATE TABLE IF NOT EXISTS project__educational(
    project_id VARBINARY(16) NOT NULL,
    educational_id VARBINARY(16) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW() ON UPDATE now(),
    PRIMARY KEY (project_id, educational_id),
    INDEX (educational_id)
);

CREATE TABLE IF NOT EXISTS crate (
    crate_id VARBINARY(16) NOT NULL,
    display_name VARCHAR(100) NOT NULL,
    description TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW() ON UPDATE now(),
    PRIMARY KEY (crate_id)
);

CREATE TABLE IF NOT EXISTS crate_category(
    category_id VARBINARY(16) NOT NULL,
    display_name VARCHAR(50) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW() ON UPDATE now(),
    PRIMARY KEY (category_id)
);

CREATE TABLE IF NOT EXISTS crate__crate_category(
    crate_id VARBINARY(16) NOT NULL,
    category_id VARBINARY(16) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW() ON UPDATE now(),
    PRIMARY KEY (crate_id, category_id)
);

CREATE TABLE IF NOT EXISTS crate__associated_crates(
    crate_id VARBINARY(16) NOT NULL,
    crate_name VARCHAR(64) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW() ON UPDATE now(),
    PRIMARY KEY (crate_id, crate_name)
);

CREATE TABLE IF NOT EXISTS crate__link(
    crate_id VARBINARY(16) NOT NULL,
    link_id VARBINARY(16) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW() ON UPDATE now(),
    PRIMARY KEY (crate_id, link_id)
);

CREATE TABLE IF NOT EXISTS crate__image(
    crate_id VARBINARY(16) NOT NULL,
    image_id VARBINARY(16) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW() ON UPDATE now(),
    PRIMARY KEY (crate_id, image_id)
);

CREATE TABLE IF NOT EXISTS project (
    project_id VARBINARY(16) NOT NULL,
    display_name VARCHAR(100) NOT NULL,
    description TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW() ON UPDATE now(),
    PRIMARY KEY (project_id)
);

COMMIT;