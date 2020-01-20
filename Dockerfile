FROM ekidd/rust-musl-builder:latest AS build

WORKDIR /usr/src/expense-calculator

COPY . .

RUN sudo chown -R rust:rust /usr/src/expense-calculator

RUN cargo build --release

CMD ["expense-calculator"]

RUN cargo build --release

FROM alpine:latest
RUN apk --no-cache add ca-certificates
COPY --from=build \
    /usr/src/expense-calculator/target/x86_64-unknown-linux-musl/release/expense-calculator \
    /usr/local/bin/

EXPOSE 8001

CMD ["/usr/local/bin/expense-calculator"]