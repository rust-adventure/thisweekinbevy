BEGIN;

CREATE UNIQUE INDEX issue_date ON issue (issue_date DESC);

COMMIT;