FROM rust:1.75 as builder

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY migrations ./migrations

RUN cargo install sqlx-cli --no-default-features --features postgres
RUN cargo sqlx prepare

ENV SQLX_OFFLINE=true

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    libpq5 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/trepi-server /app/
COPY --from=builder /usr/local/cargo/bin/sqlx /usr/local/bin/
COPY migrations ./migrations

RUN echo '#!/bin/bash\nset -e\nsqlx migrate run\nexec ./trepi-server' > start.sh && chmod +x start.sh

EXPOSE 8000

CMD ["./start.sh"]