# JWT authentication with Actix-web

This is me learning actix-web. Please suggest improvements!

## How to run

Start Postgres
```console
docker run --rm --name postgres -p 5432:5432 -e POSTGRES_PASSWORD=password -d postgres:13.1
```
and start the application
```
cargo run
```
Then off you go!

When you are done you can stop Postgres
```console
docker stop postgres
```
