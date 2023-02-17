FROM rust:latest

WORKDIR /app
RUN apt update
RUN apt upgrade -y
RUN apt install -y g++-mingw-w64-x86-64
RUN rustup target add x86_64-pc-windows-gnu
RUN rustup toolchain install stable-x86_64-pc-windows-gnu
ENV CARGO_TARGET_DIR=target/target.x86_64-windows
CMD ["cargo", "build", "--release", "--target", "x86_64-pc-windows-gnu"]
