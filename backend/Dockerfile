FROM rust:1.71 as build

ENV TZ=Europe/Stockholm
ENV DEBIAN_FRONTEND=noninteractive
RUN apt update && apt install -y tzdata musl-tools pkg-config libssl-dev

WORKDIR /app

COPY . .

RUN rustup target add x86_64-unknown-linux-musl
RUN SQLX_OFFLINE=true cargo build --release --target x86_64-unknown-linux-musl
RUN strip target/x86_64-unknown-linux-musl/release/invisibot_server



FROM scratch
WORKDIR /

ENV PORT=8080
EXPOSE 8080

COPY postgres/migrations ./
COPY --from=build /app/target/x86_64-unknown-linux-musl/release/invisibot_server ./invisibot_server
COPY --from=build /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/

CMD ["./invisibot_server"]