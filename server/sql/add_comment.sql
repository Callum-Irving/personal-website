INSERT INTO comment (post_slug, username, content, parent)
VALUES ($1, $2, $3, $4)
RETURNING username, content, posted, id;
