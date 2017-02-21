FROM jimmycuadra/rust:latest

WORKDIR /source
ADD . /source

RUN sh bin/angry-apt update
RUN sh bin/angry-apt install -y \
	curl \
	build-essential \
	libfontconfig1 \
	cmake \
	qt5-default \
	qtdeclarative5-dev

ENV PKG_CONFIG_PATH /usr/lib/x86_64-linux-gnu/pkgconfig

RUN cargo build
RUN cargo test
RUN cargo bench
RUN cargo doc

CMD [ "cargo", "run" ]
