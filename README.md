# Keylink

## Build

### Dependencies
You need to install the following dependencies:
```
sqlite-devel
```

## Test

### Keycloak
You can start Keycloak with `docker-compose up -d` and then access it at
http://localhost:8080/auth/.

Some set-up is required.
* Log in with `admin/admin`.
* Modify `Valid Redirect URIs` in `Client > account` to `*` to allow the
  redirection from Keycloak to Keylink.
* Copy the secret in `Client > account > Credentials` and put it in `client_secret`
  in the `Rocket.toml` config file.

### Keylink
* Run `cargo run` and visit http://127.0.0.1:8000.

## Contributing

### Modify the database table
First, modify `migrations/2020-11-03-101531_create_keys_table/up.sql`, and then
run:

```bash
$ diesel migration redo --database-url db.sqlite
```

Note: you need the Diesel CLI, so run:
```bash
cargo install diesel_cli --no-default-features --features sqlite
```
