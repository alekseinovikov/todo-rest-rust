FROM rust:1.65-alpine AS builder

RUN apk add --no-cache musl-dev
COPY . .
RUN cargo build --release

FROM scratch
COPY --from=builder ./target/release/todo-rest-rust ./app
COPY --from=builder ./migrations ./migrations
COPY --from=builder ./Rocket.toml ./Rocket.toml

CMD ["app"]
