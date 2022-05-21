CREATE TYPE JOB_STATUS AS ENUM ('Queued', 'Running', 'Failed');

CREATE TABLE jobs (
    id      BIGINT     NOT NULL GENERATED ALWAYS AS IDENTITY,
    status  JOB_STATUS NOT NULL DEFAULT 'Queued',
    payload JSONB NOT NULL,
    params JSONB
);