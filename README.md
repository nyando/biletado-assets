# Biletado Assets API Backend

This is a microservice implmenting the backend `assets`-API for the Biletado system.
The service is implemented in Rust using the `actix-web` framework.

## Running the Project in the Biletado Context

Clone the [nyando/compose](https://gitlab.com/nyando/compose) repository and run

```bash
docker-compose up -d
```

## Building Locally

Building the project requires the following dependencies:

- Rust (duh), preferably an up-to-date version ;)
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

## Project Structure

The entry point for the API server is `main.rs`.
In the `main` function, logger and database connection pool are initialized and the `HTTPServer` is started up.
`actix` is an actor model framework that serves as the basis for the `actix-web` framework used in this service.
Requests are processed asynchronously.

The rest of the service consists of two modules:

- the `db` module offers an interface for interacting with the database service
- the `api` module implements method handlers and access control for all HTTP requests.

### Database: The `db` Module

The database connectivity is implemented with the `diesel` ORM in the `db` module.
Connections to the DB are handled with the `r2d2` library,
which allows for creation of a thread pool to handle incoming requests,
so that connections are not created and destroyed for every request.
Functionality for establishing a database connection is located in the `db::dbconn` submodule.
`models` contains the native Rust structures corresponding to the database objects.
`schema` contains the corresponding database schema for the `assets` relations.

The CRUD functionality relating to the `assets` API is implemented in the `db::crud` submodule.
The functionality is split by the object type (`building`, `storey`, and `room`).
An analogous separation happens in the client-facing `api` module.

### Interface: The `api` Module

The `api` module contains the business logic for dealing with API requests.
The Rust framework used here is `actix-web`.

The implementations of the CRUD API for `buildings`, `storeys`, and `rooms` are located in the correspondingly named module files.
`actix-web` provides macros (`#...`) for creating HTTP routes and wrapping them in middleware modules.

The `api::auth` submodule contains handlers for validating the JWT tokens in the `HttpAuthentication` middleware.
The middleware `HttpAuthentication::bearer(validator)` in a routing macro indicates
that the operation requires authentication with a JSON web token (JWT).

The `api::util` submodule contains functions for validating the UUID inputs and extracting `Jaeger` headers from requests.
The Jaeger header key is consistent across the entire project and should be set in the `JAEGER_HEADER` environment variable.
Procedures that use other parts of the API (Keycloak or the `reservations` backend)
should add this header with the corresponding request's trace value attached to enable the `Jaeger` service to track the request.
In order for tracing to work, any requests made this way should also be routed over the reverse proxy (`traefik`).

## Dockerization

Creating a Docker container from `biletado-assets` works by
compiling a statically linked executable with the `rust-musl-builder` container.
This builder contains dependencies like the PostgreSQL driver `libpq` and the OpenSSL libary.
The resulting binary is then copied into the `scratch` empty container, where it's run.
The resulting total container size amounts to a few megabytes,
compared to the 2 GB base size of a Rust docker container.

**NOTE**: The `HttpServer` that provides the API must be bound to `0.0.0.0` instead of `localhost`,
as it will refuse outside connections otherwise (e.g. `curl` errors with `CONNRESET`).
`localhost` works for local testing, but fails in Docker (RIP my Monday evening).

### Docker Environment Variables

- `KEYCLOAK_HOST` - host address of the Keycloak authentication server (we use `traefik`, for tracing purposes)
- `KEYCLOAK_REALM` - Keycloak realm that supplies the public key for JWT authentication
- `JAEGER_HEADER` - HTTP header key of the Jaeger trace headers
- `RESERVATIONS_HOST` - host address of the `reservations` API service (we query via `traefik:80` in this case)
- `RESERVATIONS_PORT` - host port of the `reservations` API service (we query via `traefik:80` in this case)
- `POSTGRES_ASSETS_USER` - username of the PostgreSQL database server
- `POSTGRES_ASSETS_PASSWORD` - username of the PostgreSQL database server
- `POSTGRES_ASSETS_DBNAME` - database name on the PostgreSQL database server (`assets`)
- `POSTGRES_ASSETS_HOST` - host address of the PostgreSQL database server
- `POSTGRES_ASSETS_PORT` - port for accessing the PostgreSQL service
- `RUST_LOG` - set the log level: `error`, `warn`, `info`, `debug`, `trace`

## CI/CD Pipeline

`biletado-assets` uses _GitHub Actions_ as a continuous integration system.
The current pipeline compiles the project and pushes a Docker container to the `docker.io` repository.
The pipeline also runs unit tests and fails the build if these do not pass.

The unit tests are not particularly interesting,
but function more as a proof-of-work for working integration in the CI/CD system.
Once integration tests are available they might be integrated (_hehe_) here too.

## Logging

`biletado-assets` uses `env_logger` for logging to `stdout`.
The log level is specifiable via the `RUST_LOG` environment variable.
The libraries and/or frameworks used in this project use the same logging utility,
so most `debug`/`info`/`warning`/`error` messages will probably come from other sources.
