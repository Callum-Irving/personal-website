SELECT username, content, posted
FROM comment
WHERE post_slug=$1;
