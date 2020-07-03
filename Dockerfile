FROM ekidd/rust-musl-builder:stable as builder

ENV PKG_CONFIG_ALLOW_CROSS=1

WORKDIR /home/rust/src
RUN sudo chown -R rust:rust /home/rust/src

ADD . .

RUN cargo build --release

# ------------------------------------------------------------------------------

FROM alpine:latest
RUN apk --no-cache add ca-certificates

COPY --from=builder \
  /home/rust/src/target/x86_64-unknown-linux-musl/release/discord-bot \
  /usr/local/bin/

CMD /usr/local/bin/discord-bot
