INSERT INTO comment (post_id, username, content)
VALUES ($1, $2, $3)
RETURNING username, content, posted;
