FROM rust:1-stretch as builder
# Choose a workdir
WORKDIR /usr/src/app
# Create blank project
RUN USER=root cargo init
# Copy Cargo.toml to get dependencies
COPY Cargo.toml .
COPY src src
RUN apt-get update && apt-get install -y musl-tools
RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --target x86_64-unknown-linux-musl --release
RUN strip target/x86_64-unknown-linux-musl/release/kubia

FROM scratch
# Copy bin from builder to this new image
COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-musl/release/kubia /kubia
# Default command, run app
CMD ["/kubia"]