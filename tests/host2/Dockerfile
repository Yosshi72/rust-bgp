FROM rust:1-buster

WORKDIR /rust-bgp
COPY . .
RUN cargo add regex --features unicode-case
RUN rustup default nightly && cargo build
CMD ["./target/debug/rust-bgp", "64513 10.200.100.3 65412 10.200.100.2 passive"]