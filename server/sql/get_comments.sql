SELECT username, content, posted, parent, id
FROM comment
WHERE post_slug=$1
ORDER BY parent ASC, id DESC;
