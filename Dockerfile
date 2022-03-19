ARG BASE_IMAGE=ekidd/rust-musl-builder:latest

FROM ${BASE_IMAGE} AS build
ADD --chown=rust:rust . ./
RUN cargo build --release

FROM scratch
EXPOSE 8081
COPY --from=build /home/rust/src/target/x86_64-unknown-linux-musl/release/biletado-assets /
USER 1000
CMD ["/biletado-assets"]