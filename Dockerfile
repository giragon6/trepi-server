# build

FROM rust:1.85 as builder

WORKDIR /app
COPY . .

RUN cargo build --release

# deploy

FROM gcr.io/distroless/cc-debian12:latest
COPY --from=builder /app/target/release/trepi-server /usr/local/bin/trepi-server
WORKDIR /app
ENV DATABASE_URL=${DATABASE_URL}
CMD ["sh", "-c", "trepi-server"]