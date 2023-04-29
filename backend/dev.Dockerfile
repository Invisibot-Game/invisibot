FROM rust:1.69

ENV TZ=Europe/Stockholm
ENV DEBIAN_FRONTEND=noninteractive

WORKDIR /app

RUN cargo install cargo-watch

ENV PORT=8080
EXPOSE 8080

CMD cargo watch -x run
