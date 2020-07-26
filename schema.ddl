DROP TABLE IF EXISTS users;
CREATE TABLE users (
    id  INTEGER PRIMARY KEY,
    username TEXT  NOT NULL UNIQUE
);

CREATE UNIQUE INDEX uname on users(username);

DROP TABLE IF EXISTS items;
CREATE TABLE items (
    id  INTEGER PRIMARY KEY,
    title TEXT NOT NULL,
    body TEXT NOT NULL,
    done BOOL NOT NULL DEFAULT false
);

DROP TABLE IF EXISTS votes;
CREATE TABLE votes (
    user_id INTEGER NOT NULL,
    item_id INTEGER NOT NULL,
    ordinal INTEGER NOT NULL,
    
    PRIMARY KEY (user_id, item_id),
    FOREIGN KEY(user_id) REFERENCES users(id),
    FOREIGN KEY(item_id) REFERENCES items(id)
);

CREATE UNIQUE INDEX no_dup_votes on votes(user_id, item_id);
CREATE UNIQUE INDEX ballot on votes(user_id ASC, ordinal ASC);

INSERT INTO items (title, body) 
VALUES ("Async SSH part 2", "Are the animals restless?");

INSERT INTO items (title, body) 
VALUES ("Tokio Zookeeper Cleanup", "Zookeeper is awesome!");

INSERT INTO items (title, body) 
VALUES ("Tokio Futures Extravaganza", "Everything you wanted to know about Tokio Futures.");


INSERT INTO users (username) VALUES ("bob");
INSERT INTO users (username) VALUES ("gopinder@foo.com");
INSERT INTO users (username) VALUES ("farook@bar.com");
INSERT INTO users (username) VALUES ("vitalik@baz.com");

INSERT INTO votes (user_id, item_id, ordinal)
VALUES ("1", "1", "1");


