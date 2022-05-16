INSERT INTO comment (post_slug, username, content)
VALUES ($1, $2, $3)
RETURNING username, content, posted;
