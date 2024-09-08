FROM rust:latest AS builder
WORKDIR /usr/src/myapp
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim AS runner
RUN apt-get update && apt-get -y upgrade
COPY --from=builder /usr/src/myapp/target/release/rust-webserver /usr/local/bin/rust-webserver
COPY --from=builder /usr/src/myapp/data/dummy_data.csv /usr/local/bin/data/dummy_data.csv

EXPOSE 3000
ENTRYPOINT [ "/usr/local/bin/rust-webserver" ]

