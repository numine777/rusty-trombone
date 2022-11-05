DROP TABLE IF EXISTS play_queue;

CREATE TABLE play_queue (
  id serial PRIMARY KEY,
  user_id BIGINT NULL,
  note TEXT NOT NULL
);
