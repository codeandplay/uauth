FROM liuchong/rustup

RUN rustup default nightly && rustup update

USER root

RUN mkdir -p /home/rocket/.cargo/target/release

ENV CARGO_TARGET_DIR=/app/target

RUN apt-get update
RUN apt-get install libpq-dev -y

WORKDIR /app

COPY Cargo.toml Cargo.toml
COPY diesel.toml diesel.toml
COPY src src
COPY migrations migrations


RUN cargo build --target x86_64-unknown-linux-gnu --release 

CMD ["cargo", "run"]


