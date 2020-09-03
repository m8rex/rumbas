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
RUN cd Numbas && git checkout f420421a7ef3c2cd4c39e43f377d2a363ae2f81e
RUN cd Numbas && pip install -r requirements.txt

# Add jsx graph extension
RUN mkdir -p Numbas/extensions
RUN git clone https://github.com/numbas/numbas-extension-jsxgraph.git Numbas/extensions/jsxgraph
RUN cd Numbas/extensions/jsxgraph && git checkout 9bc865f695009cf1942060be4e725e3dc687895b 

ENV NUMBAS_FOLDER=/usr/app/Numbas

COPY --from=builder /usr/app/rumbas/target/x86_64-unknown-linux-musl/release/rumbas /bin/rumbas
COPY entrypoint.sh .
WORKDIR /rumbas
ENTRYPOINT ["/usr/app/entrypoint.sh"]
