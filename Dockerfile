FROM rust:alpine AS builder

WORKDIR /src/builder

COPY . .
RUN cargo build --release

FROM scratch

WORKDIR /src/app

COPY --from=builder /src/builder/target/release/shorten .

CMD ["./shorten"]
