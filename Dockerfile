FROM rust:1.43.1-slim-stretch as builder

WORKDIR /usr/app
RUN rustup target add x86_64-unknown-linux-musl

#use tricks of https://shaneutt.com/blog/rust-fast-small-docker-image-builds/
RUN mkdir numbas
COPY numbas/Cargo* numbas/
COPY numbas/src numbas/src

RUN mkdir -p rumbas/src
COPY rumbas/Cargo* rumbas/
RUN echo "fn main() {println!(\"if you see this, we are rebuilding the dependencies of rumbas\")}" > rumbas/src/main.rs
RUN cd rumbas && cargo build --target=x86_64-unknown-linux-musl --release
RUN rm -f rumbas/target/x86_64-unknown-linux-musl/release/deps/rumbas*

COPY rumbas/src rumbas/src
RUN cd rumbas && cargo build --target=x86_64-unknown-linux-musl --release

FROM python:3.6.10-alpine 

WORKDIR /usr/app
RUN apk add git
RUN git clone https://github.com/numbas/Numbas.git Numbas

WORKDIR /usr/app/Numbas
RUN git fetch && git checkout 58fcacacefd393518645f88b17a44c55da27810b
RUN pip install -r requirements.txt

# Add jsx graph extension
RUN mkdir -p extensions
WORKDIR /usr/app/Numbas/extensions
RUN git clone https://github.com/numbas/numbas-extension-jsxgraph.git jsxgraph
WORKDIR /usr/app/Numbas/extensions/jsxgraph
RUN git fetch && git checkout 9bc865f695009cf1942060be4e725e3dc687895b 

ENV NUMBAS_FOLDER=/usr/app/Numbas

COPY --from=builder /usr/app/rumbas/target/x86_64-unknown-linux-musl/release/rumbas /bin/rumbas
COPY entrypoint.sh .
WORKDIR /rumbas
ENTRYPOINT ["/usr/app/entrypoint.sh"]
