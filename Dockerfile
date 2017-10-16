FROM rust:1.19.0

WORKDIR /usr/src/githubspot
COPY . .

RUN cargo install

CMD ["githubspot"]
