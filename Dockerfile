FROM rust

WORKDIR /usr/src/swhook
COPY src ./src
COPY Cargo.lock Cargo.lock
COPY Cargo.toml Cargo.toml

RUN cargo build --release --locked
WORKDIR target/release

CMD ["./swhook", "server"]
