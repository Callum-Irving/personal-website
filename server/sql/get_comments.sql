SELECT username, content, posted, id
FROM comment
WHERE post_slug=$1
ORDER BY id DESC;
