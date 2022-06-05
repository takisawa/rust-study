FROM rust:1.61.0-bullseye as build-env
WORKDIR /app
COPY . /app
RUN cargo build --release

FROM gcr.io/distroless/cc
COPY --from=build-env /app/target/release/distroless_hello_world /
CMD ["/distroless_hello_world"]
