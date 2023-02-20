FROM rust:latest

WORKDIR /usr/src
COPY . . 

RUN cargo install --path .
EXPOSE 8080

CMD ["cargo", "run"]