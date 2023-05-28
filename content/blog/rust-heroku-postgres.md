+++
title = "Connecting Heroku Postgres to Rust"
date = 2022-05-18
description = "A guide on using Heroku Postgres with Rust."
+++

I was learning Postgres and Actix Web recently to create a comment system for
this blog and it ended up being a huge pain to connect the Heroku Postgres
database to my Actix Web application. Inspired by that pain, I have written this
guide.

# Dependencies

This is going to require a few crates. Here is a list of dependencies in
`Cargo.toml`:

```toml
[dependencies]
url = "2.2.2"
tokio-postgres = "0.7.6"
deadpool-postgres = "0.10.2"
postgres-openssl = "0.5.0"
openssl = "0.10.40"
```

Here's what all these dependencies do:

- `url`: Used to parse database info from the URL given by Heroku.
- `tokio-postgres`: An async client for Postgres in Rust.
- `deadpool-postgres`: Async pool for database connections.
- `postgres-openssl`: SSL support for Postgres.
- `openssl`: SSL support for Rust.

You'll also need some sort of async runtime. For my project I was using Actix
Web which runs on Tokio but for simplicity I'm just going to use Tokio for this
post.

```toml
tokio = { version = "1.18.2", features = ["full"] }
```

Now that we have our dependencies added, we're ready to go!

# Connecting to Postgres

## Generating a `deadpool_postgres` config

When you create a Postgres database in Heroku, it will set an environment
variable in your project called `DATABASE_URL`. Unfortunately we can't use this
URL directly so we are going to parse it.

Here's what the function to parse it looks like:

```rust
fn config_from_url(url: &str) -> Result<deadpool_postgres::Config, &'static str> {
    // Config URL is in format:
    // postgres://USER:PASSWORD@HOST:PORT/DBNAME
    let url = Url::parse(url).map_err(|_| "could not parse db url")?;
    let user = url.username().to_string();
    let password = url.password().ok_or("bad password")?.to_string();
    let host = url.host().ok_or("bad host")?.to_string();
    let port = url.port().ok_or("no port in url")?;
    let dbname = url
        .path_segments()
        .ok_or("cannot be base")?
        .next()
        .unwrap()
        .to_string();

    let mut cfg = deadpool_postgres::Config::new();
    cfg.user = Some(user);
    cfg.password = Some(password);
    cfg.host = Some(host);
    cfg.port = Some(port);
    cfg.dbname = Some(dbname);

    Ok(cfg)
}
```

Let's give it a try!

```rust
#[tokio::main]
async fn main() {
    let database_url = std::env::var("DATABASE_URL").unwrap();
    let cfg = config_from_url(&database_url).unwrap();
    dbg!(&cfg);
}
```

The output you get should look something like this:

```
[src/main.rs:35] &cfg = Config {
    user: Some(
        "[user]",
    ),
    password: Some(
        "[password]",
    ),
    dbname: Some(
        "[database name]",
    ),
    options: None,
    application_name: None,
    ssl_mode: None,
    host: Some(
        "[link]",
    ),
    hosts: None,
    port: Some(
        [port],
    ),
    ports: None,
    connect_timeout: None,
    keepalives: None,
    keepalives_idle: None,
    target_session_attrs: None,
    channel_binding: None,
    manager: None,
    pool: None,
}
```

With all the values in the square brackets being replaced by the values from the
Heroku URL.

## Setting up SSL

This was the biggest headache for me but it's actually very simple. The thing
with Heroku Postgres is that it requires SSL, but doesn't give certificates. To
make this work we need to set the SSL verify mode to off in our SSL config in Rust.

```rust
let mut builder = SslConnector::builder(SslMethod::tls()).unwrap();
builder.set_verify(SslVerifyMode::NONE);
let connector = MakeTlsConnector::new(builder.build());
```

And that's all the SSL stuff we need to do.

## Creating a connection pool

Creating a connection pool with `deadpool_postgres` is just one line of code:

```rust
let pool = cfg.create_pool(None, connector).unwrap();
```

And that's it for the setup! Now we can query the database.

# Querying the database

## Creating a table

Before we can query anything we need to create a table in our database. If you
have already done this just skip to the next step.

I'm going to make a directory for all our SQL files in the root of the project.
The structure should now look like this:

```
.
├── .gitignore
├── Cargo.lock
├── Cargo.toml
├── sql
└── src
    └── main.rs
```

For creating the table I'll make a file called `schema.sql` in the sql
directory. For this example we'll make a simple blog system. We will store the
id, author, date, title, and content of each post.

The SQL code to create such a table looks like this:

```sql
CREATE TABLE post (
    id SERIAL PRIMARY KEY,
    posted DATE NOT NULL DEFAULT CURRENT_DATE,
    author VARCHAR(50) NOT NULL,
    title VARCHAR(255) NOT NULL,
    content TEXT NOT NULL
);
```

We can write a function in our Rust code to call this SQL query.

```rust
// Add this at the top of your file
use deadpool_postgres::Client;

async fn create_table(client: &Client) {
    let stmt = include_str!("../sql/schema.sql");
    let stmt = client.prepare(stmt).await.unwrap();

    client.query(&stmt, &[]).await.unwrap();
}
```

I'm just going to put this at the bottom of my main function, run it once, then
remove it. You could also use the psql command line tool instead of all of this
if you prefer.

## Inserting some data

Now lets make another SQL file to add a post. I'll call it `add_post.sql`.

```sql
INSERT INTO post (author, title, content)
VALUES ($1, $2, $3);
```

We omit the date field because Postgres will default it to the current date. The
numbers prefixed by $ will be replaced in our Rust code.

Now lets make a struct to represent one post.

```rust
struct Post {
    author: String,
    title: String,
    content: String,
}
```

Now that we have the SQL to add a post and the Rust struct, we can write a
function that inserts a `Post` into the database.

```rust
async fn add_post(client: &Client, post: Post) {
    let stmt = include_str!("../sql/add_post.sql");
    let stmt = client.prepare(stmt).await.unwrap();

    let res = client
        .query(&stmt, &[&post.author, &post.title, &post.content])
        .await
        .unwrap();

    dbg!(res);
}
```

## Getting data

Getting data is very similar to inserting data. Say we want to get all the posts
where the author is "John Doe". First we need to build an SQL query. I'll make a
file called `get_posts.sql`.

```sql
SELECT * FROM post
WHERE author=$1
ORDER BY posted DESC;
```

Now we make a Rust function very similar to the one used to add a new post:

```rust
async fn get_posts(client: &Client, author: &str) {
    let stmt = include_str!("../sql/get_posts.sql");
    let stmt = client.prepare(stmt).await.unwrap();

    let res = client
        .query(&stmt, &[&author])
        .await
        .unwrap();

    dbg!(res);
}
```

# Conclusion

These are the basic building blocks you need to use Heroku Postgres from Rust.
Hopefully this was helpful! Please leave a comment if something in this post
needs to be clarified.

If you plan to use any of this code you should **DEFINITELY** add error handling
instead of just unwrapping everything!

If you want to see the project that I used this in, check out
[this old commit of my website](https://github.com/Callum-Irving/personal-website/tree/2fe86cd43390cd0c2f0e173659509578ff218cd4/server). I actually ended up getting rid of the backend in a later commit and switching to a different comment system but the code works fine.
