FROM rust:slim-bullseye AS builder

WORKDIR /myapp
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
RUN apt-get update && rm -rf /var/lib/apt/lists/*

WORKDIR /myapp
COPY --from=builder /myapp/target/release/idp-console .
COPY --from=builder /myapp/schema.sql .
COPY dist ./dist

CMD ["./idp-console"]