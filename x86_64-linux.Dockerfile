FROM amd64/rust:latest

WORKDIR /app
COPY . .
RUN apt update
RUN apt upgrade -y
RUN apt install -y g++-aarch64-linux-gnu libc6-dev-arm64-cross libxcb1-dev libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev
RUN rm -rf /var/lib/apt/lists/*
RUN cargo install --path .
ENV CARGO_TARGET_DIR=target.x86_64-linux
CMD ["cargo", "build", "--release"]
