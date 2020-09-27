FROM rust:alpine AS builder
RUN apk add musl-dev
ADD . /code
RUN (cd /code; cargo build --release)

FROM alpine
COPY --from=builder /code/target/release/calbox /calbox
CMD ["/calbox"]
