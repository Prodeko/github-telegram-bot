FROM rust:1.79 as build-stage

WORKDIR /webbitiimibot

RUN rustup target add x86_64-unknown-linux-musl
RUN apt-get update && apt-get install musl-tools -y
RUN USER=root cargo new --bin webbitiimibot

COPY . .
RUN --mount=type=cache,target=/usr/local/cargo,from=rust:1.79,source=/usr/local/cargo \
    --mount=type=cache,target=target \
    cargo build --release --target x86_64-unknown-linux-musl && cp target/x86_64-unknown-linux-musl/release/webbitiimibot output_binary

FROM scratch

WORKDIR /webbitiimibot

COPY --from=build-stage /webbitiimibot/output_binary /webbitiimibot/output_binary
CMD /webbitiimibot/output_binary