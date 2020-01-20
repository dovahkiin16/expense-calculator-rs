FROM ekidd/rust-musl-builder:latest AS build

WORKDIR /usr/src/expense-calculator

COPY . .

RUN sudo chown -R rust:rust /usr/src/expense-calculator

RUN cargo build --release

# Size optimization
RUN strip target/x86_64-unknown-linux-musl/release/expense-calculator

FROM scratch

WORKDIR /usr/expense-calculator
#RUN apk --no-cache add ca-certificates
COPY --from=build \
    /usr/src/expense-calculator/target/x86_64-unknown-linux-musl/release/expense-calculator .

EXPOSE 8001

ENTRYPOINT ["./expense-calculator"]
