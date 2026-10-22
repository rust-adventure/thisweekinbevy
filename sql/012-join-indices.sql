BEGIN;

CREATE INDEX image_id ON crate_release__image (image_id);

CREATE INDEX showcase_id ON showcase__image (showcase_id);

COMMIT;