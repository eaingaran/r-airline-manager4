FROM alpine:latest as alpine

RUN apk add -U --no-cache ca-certificates

# source    - https://github.com/emk/rust-musl-builder/blob/main/Dockerfile
# dockerhub - https://hub.docker.com/r/ekidd/rust-musl-builder
FROM ekidd/rust-musl-builder:latest AS builder

RUN rustup update

COPY . .

RUN cargo build --release --target x86_64-unknown-linux-musl

# working directory of the builder image must be considered (/home/rust/src)
# check the source for more info
FROM scratch
COPY --from=builder /home/rust/src/target/x86_64-unknown-linux-musl/release/airline_manager4 /bin/airline_manager4
COPY --from=alpine /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/

CMD ["/bin/airline_manager4"]