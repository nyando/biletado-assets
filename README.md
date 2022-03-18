# Biletado Assets API Backend

This is a microservice implmenting the backend `assets`-API for the Biletado system.

## Build

The service is developed in Rust and uses the `cargo` build tool.
To fetch dependencies and build the project, run the following command in the project directory:

```bash
cargo build
```

The following command builds and runs the project:

```bash
cargo run
```

## Submodules

The database connectivity is implemented with `diesel` ORM in the `db` module.
`models` contains the native Rust structures corresponding to the database objects.
`schema` contains the corresponding database schema for the `assets` relations.

The `api` module contains the business logic for dealing with API requests.
The Rust framework used here is `actix`.
Currently, a skeleton is implemented for all endpoints specified in the `assets` OpenAPI definition,
but the functionality is yet to be implemented.
