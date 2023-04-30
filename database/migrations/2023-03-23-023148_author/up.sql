CREATE TABLE author (
    authorid SERIAL PRIMARY KEY,
    username VARCHAR(100) NOT NULL,
    firstname VARCHAR(100) NOT NULL,
    lastname VARCHAR(100) NOT NULL,
    email VARCHAR(50) NOT NULL,
    UNIQUE(email)
);