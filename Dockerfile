FROM jimmycuadra/rust:latest

WORKDIR /source
ADD bin/angry-apt /usr/bin/angry-apt
RUN chmod +x /usr/bin/angry-apt

RUN angry-apt update
RUN angry-apt install -y \
	curl \
	build-essential \
	libfontconfig1 \
	cmake \
	qt5-default \
	qtdeclarative5-dev

ENV PKG_CONFIG_PATH /usr/lib/x86_64-linux-gnu/pkgconfig

ADD Cargo.toml /source/Cargo.toml
RUN cargo fetch

ADD src /source/src
RUN \
cargo build && \
cargo test && \
cargo bench && \
cargo doc

CMD [ "cargo", "run" ]
