FROM rust:1.89

WORKDIR /app

COPY Cargo.toml Cargo.lock build.rs ./
COPY src/ ./src/
COPY benches/ ./benches/

RUN cargo build --release --bin zjyo

RUN mkdir -p /test-dirs/project/src /test-dirs/documents/reports /test-dirs/downloads/tools

RUN cp target/release/zjyo /usr/local/bin/z

COPY test.sh /test.sh
RUN chmod +x /test.sh

WORKDIR /test-dirs

CMD ["bash"]
