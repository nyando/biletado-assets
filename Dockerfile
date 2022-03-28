# Not an official(TM) Docker(TM) image(TM),
# but oh man this image is awesome.
# It's like it was designed for our use case.
ARG BASE_IMAGE=ekidd/rust-musl-builder:latest
FROM ${BASE_IMAGE} AS build
ADD --chown=rust:rust . ./
RUN cargo build --release

# "Runtime Environment? What's that?"
# The download is like 6 MB,
# the uncompressed image is ~20 MB.
FROM scratch
EXPOSE 9000
COPY --from=build /home/rust/src/target/x86_64-unknown-linux-musl/release/biletado-assets /
USER 1000
CMD ["/biletado-assets"]
