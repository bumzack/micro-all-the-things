FROM rust:1.68.2 AS builder
COPY . .
RUN cargo build --release

FROM ubuntu:latest
COPY --from=builder ./target/release/rust_crewwriter ./target/release/rust_crewwriter
CMD ["/target/release/rust_crewwriter"]
