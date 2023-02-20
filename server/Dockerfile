FROM rust:latest

WORKDIR /usr/src
COPY . . 

RUN cargo install --path .

CMD ["cargo", "run"]