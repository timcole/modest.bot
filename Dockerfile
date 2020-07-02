FROM rust:latest as cargo-build

RUN apt-get update
RUN apt-get install musl-tools -y
RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /usr/src/discord-bot
ADD . /usr/src/discord-bot

RUN cargo build --release --target=x86_64-unknown-linux-musl

# ------------------------------------------------------------------------------

FROM alpine:latest

WORKDIR /usr/src/discord-bot
COPY --from=cargo-build /usr/src/discord-bot/target/x86_64-unknown-linux-musl/release/discord-bot .

CMD ["./discord-bot"]
