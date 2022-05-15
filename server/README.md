Setup database:

```sh
PGPASSWORD={PASSWORD} psql -h {HOST} -U {USER} -d {DBNAME} < {SQL FILE PATH}
```

Reset database:

```sh
herok pg:reset DATABASE
```

Get info:

```sh
heroku pg:credentials:url DATABASE
```

To find credentials:

- Go to the Heroku web page
- Go to resources
- Click postgres
- Click settings
- Click credentials
