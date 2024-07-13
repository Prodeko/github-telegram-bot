FROM rust:1.79

RUN rustup target add x86_64-unknown-linux-musl
RUN apt-get update && apt-get install musl-tools -y    

RUN rustup component add rustfmt
RUN USER=root cargo new --bin webbitiimibot
WORKDIR /webbitiimibot
RUN git config --global --add safe.directory /webbitiimibot

# Install SQLx CLI for database migrations (see README)
RUN cargo install sqlx-cli --no-default-features --features native-tls,postgres
# No need to copy or build anything in dev container
# COPY ./Cargo.* .
# RUN cargo build
# COPY . .

