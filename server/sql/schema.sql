DROP TABLE IF EXISTS comment;

CREATE TABLE comment (
    id SERIAL PRIMARY KEY,
    post_slug VARCHAR(50) NOT NULL,
    parent INT,
    posted DATE NOT NULL DEFAULT CURRENT_DATE,
    username VARCHAR(50) NOT NULL,
    content TEXT NOT NULL
);
