BEGIN;

CREATE INDEX image_id ON crate__image (image_id);

CREATE INDEX image_id ON showcase__image (image_id);

CREATE INDEX image_id ON devlog__image (image_id);

CREATE INDEX image_id ON educational__image (image_id);

CREATE TABLE IF NOT EXISTS issue__image(
    issue_id VARBINARY(16) NOT NULL,
    image_id VARBINARY(16) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW() ON UPDATE now(),
    PRIMARY KEY (issue_id, image_id),
    KEY (image_id)
);

CREATE TABLE IF NOT EXISTS project__image(
    project_id VARBINARY(16) NOT NULL,
    image_id VARBINARY(16) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW() ON UPDATE now(),
    PRIMARY KEY (project_id, image_id),
    KEY (image_id)
);

COMMIT;