CREATE TABLE users(
    id BLOB(16) NOT NULL,
    mail TEXT NOT NULL,
    password TEXT NOT NULL,
    PRIMARY KEY (id, mail)
);
CREATE TABLE tokens(
    access_token VARCHAR(65535) NOT NULL PRIMARY KEY,
    client_token VARCHAR(255) NOT NULL,
    bound_profile BLOB(16) NOT NULL,
    issue_time TIMESTAMP NOT NULL
);