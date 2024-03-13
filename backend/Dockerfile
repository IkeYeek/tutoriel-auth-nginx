FROM rust:1.76-alpine as builder
WORKDIR /usr/src/backend
COPY . .
RUN apk add musl-dev
RUN cargo install --path .

FROM debian:bullseye-slim
COPY --from=builder /usr/local/cargo/bin/backend /usr/local/bin/backend
EXPOSE 8080
CMD ["backend"]