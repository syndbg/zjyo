FROM rust:1.83

WORKDIR /app

COPY Cargo.toml ./
COPY src/ ./src/

RUN cargo build --release

RUN mkdir -p /test-dirs/project/src /test-dirs/documents/reports /test-dirs/downloads/tools

RUN cp target/release/zjyo /usr/local/bin/z

COPY test.sh /test.sh
RUN chmod +x /test.sh

WORKDIR /test-dirs

CMD ["bash"]