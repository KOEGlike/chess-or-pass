CREATE TABLE users (
    id VARCHAR PRIMARY KEY,
    username VARCHAR(100) NOT NULL,
    password bytea[] NOT NULL,
    salt VARCHAR(100) NOT NULL
);

CREATE TYPE piece AS ENUM ('bb','bk','bn','bp','bq','br','wb','wk','wn','wp','wq','wr');
CREATE TYPE voted_for AS ENUM ('first_piece', 'second_piece');

CREATE TABLE votes (
    id VARCHAR PRIMARY KEY,
    user_id VARCHAR REFERENCES users(id),
    first_piece piece NOT NULL,
    second_piece piece NOT NULL,
    voted_for voted_for NOT NULL,
    reason VARCHAR(255)
)
