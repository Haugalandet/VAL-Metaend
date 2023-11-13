FROM rust:1.73.0

WORKDIR /src

COPY . .

EXPOSE 6969

RUN cargo build

CMD [ "cargo", "run" ]