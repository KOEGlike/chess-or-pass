CREATE TABLE users (
    id VARCHAR PRIMARY KEY,
    username VARCHAR(100) NOT NULL UNIQUE,
    password bytea[] NOT NULL,
    salt VARCHAR(100) NOT NULL
);

CREATE TYPE pieces AS ENUM ('bb','bk','bn','bp','bq','br','wb','wk','wn','wp','wq','wr');

CREATE TABLE votes (
    id VARCHAR PRIMARY KEY,
    user_id VARCHAR REFERENCES users(id) NOT NULL,
    first_piece pieces NOT NULL,
    second_piece pieces NOT NULL,
    voted_for_first boolean NOT NULL,
    reason VARCHAR(255),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL
)
