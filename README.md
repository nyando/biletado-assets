# Biletado Assets API Backend

This is a microservice implmenting the backend `assets`-API for the Biletado system.

## Build

Building the project requires the following dependencies:

- PostgreSQL
  - Linux: `libpq`
  - Windows:
    - install the [PostgreSQL application](https://www.postgresql.org/download/)
    - put the PostgreSQL subfolders `bin` and `lib` folders in your `PATH`
    - create the environment variable `PQ_LIB_DIR` and set it to the PSQL `lib` folder
- OpenSSL
  - Linux: `libopenssl` or similar
  - Windows: [Follow these instructions](https://stackoverflow.com/a/61921362)

The service is developed in Rust and uses the `cargo` build tool.
To fetch dependencies and build the project, run the following command in the project directory:

```bash
cargo build  # builds the project, output to folder "target"
cargo run    # runs the project using .env as environment
```

## Database: The `db` Module

The database connectivity is implemented with `diesel` ORM in the `db` module.
Connections are handled with the `r2d2` library,
which allows for creation of a thread pool to handle incoming requests,
so that connections are not created and destroyed for every request.
`models` contains the native Rust structures corresponding to the database objects.
`schema` contains the corresponding database schema for the `assets` relations.

The CRUD functionality relating to the `assets` API is implemented in the `crud` submodule of the `db` module.
The functionality is split by the object type (`building`, `storey`, and `room`).
An analogous separation happens in the client-facing `api` module.

## Interface: The `api` Module

The `api` module contains the business logic for dealing with API requests.
The Rust framework used here is `actix`.
The API is mostly implemented, as of yet a call to the `reservations` API
to check a room for existing reservations is still TODO.

## Dockerization

Creating a Docker container from `biletado-assets` works by
compiling a statically linked executable with the `rust-musl-builder` container.
This builder contains dependencies like the PostgreSQL driver `libpq` and the OpenSSL libary.
The resulting binary is then copied into the `scratch` empty container, where it's run.
The resulting total container size amounts to a few megabytes,
compared to the 2 GB base size of a Rust docker container.

**NOTE**: The `HttpServer` that provides the API must be bound to `0.0.0.0` instead of `localhost`,
as it will refuse outside connections otherwise (TCP `56 connection refused`).
`localhost` works for local testing, but fails in Docker (RIP my Monday evening).

## CI/CD Pipeline

`biletado-assets` uses _GitHub Actions_ as a continuous integration system.
The current pipeline compiles the project and pushes a Docker container to the `docker.io` repository.
Once unit tests are implemented for some part of the project, these need to be run as well,
with the pipeline terminating if the tests do not pass.
