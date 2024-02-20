BEGIN;

ALTER TABLE
    issue
MODIFY
    cloudinary_public_id VARCHAR(200) NOT NULL DEFAULT "";

ALTER TABLE
    image
MODIFY
    cloudinary_public_id VARCHAR(200) NOT NULL DEFAULT "";

COMMIT;