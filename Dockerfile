FROM jimmycuadra/rust:latest

VOLUME /source
WORKDIR /source

RUN cargo build

CMD [ "cargo", "run" ]
