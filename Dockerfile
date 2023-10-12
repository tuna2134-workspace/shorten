FROM rust:alpine AS builder

WORKDIR /src/builder

COPY . .
RUN cargo build --target=x86_64-unknown-linux-musl --release

FROM scratch

WORKDIR /src/app

COPY --from=builder /src/builder/target/x86_64-unknown-linux-musl/release/shorten .

CMD ["./shorten"]
