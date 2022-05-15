SELECT username, content, posted
FROM comment
WHERE post_id=$1;
