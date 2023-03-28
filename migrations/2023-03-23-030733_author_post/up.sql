CREATE TABLE author_post (
    id SERIAL PRIMARY KEY,
    authorid INT NOT NULL,
    postid INT NOT NULL,
    UNIQUE(authorid, postid),
    CONSTRAINT fk_postid FOREIGN KEY(postid) REFERENCES post(postid),
    CONSTRAINT fk_authorid FOREIGN KEY(authorid) REFERENCES author(authorid) ON DELETE CASCADE
);