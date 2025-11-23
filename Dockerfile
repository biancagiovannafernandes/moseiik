FROM rust:1.85-slim
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
#  copier le code source
COPY src ./src
COPY assets ./assets
COPY tests ./tests

#RUN cargo test --release 
ENTRYPOINT ["cargo", "test", "--release", "--"]
