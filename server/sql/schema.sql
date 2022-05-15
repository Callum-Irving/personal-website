DROP TABLE IF EXISTS comment;

CREATE TABLE comment (
    id SERIAL PRIMARY KEY,
    post_id INT NOT NULL,
    username VARCHAR(50) NOT NULL,
    content TEXT NOT NULL,
    posted DATE NOT NULL DEFAULT CURRENT_DATE
);
