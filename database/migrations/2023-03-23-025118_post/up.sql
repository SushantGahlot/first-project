CREATE TABLE post (
    body VARCHAR(5000),
    created TIMESTAMP DEFAULT current_timestamp,
    postid SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL,
    updated TIMESTAMP 
);