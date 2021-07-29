![keylink header](/docs/keylinkheader.png)

[![](https://img.shields.io/badge/License-Apache--2.0-green)](https://github.com/spruceid/keylink/blob/main/LICENSE) [![](https://img.shields.io/twitter/follow/sprucesystems?label=Follow&style=social)](https://twitter.com/sprucesystems)

# Keylink

Keylink is an in-development tool that links system accounts to keys. 
Accounts are authenticated using combinations of widely adopted protocols 
such as OpenID Connect and FIDO2. Keys can range from cryptographic 
keys to API credentials. Keylink can gradually bootstrap PKI within 
existing complex IT systems. It supports a centralized PKI operating 
mode that can evolve into decentralized PKI, and further coordinates 
with existing PKI and KMS installations.

## Build

### Dependencies
You need to install the following dependencies:
```
postgres-devel
```
> libpq on Homebrew

## Test

Necessary services are provided in the docker-compose:
```bash
$ docker-compose up -d
```

### OIDC
A test user is provided with the credentials `user1/pwd`.

### Keylink
* Run `cargo run` and visit http://127.0.0.1:8000.

## Contributing

### Modify the database table
First, modify `migrations/2020-11-03-101531_create_keylink/up.sql`, and then
run:

```bash
$ diesel migration redo --database-url postgres://postgres:postgres@localhost/keylink
```

Note: you need the Diesel CLI, so run:
```bash
cargo install diesel_cli --no-default-features --features postgres
```
