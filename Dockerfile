FROM rust:1.53.0-slim as builder

WORKDIR /usr/app
RUN rustup target add x86_64-unknown-linux-musl

#use tricks of https://shaneutt.com/blog/rust-fast-small-docker-image-builds/
RUN mkdir numbas
COPY numbas/Cargo* numbas/
COPY numbas/src numbas/src

RUN mkdir -p rumbas/src
COPY rumbas/Cargo* rumbas/
RUN echo "fn main() {println!(\"if you see this, we are rebuilding the dependencies of rumbas\")}" > rumbas/src/main.rs
RUN echo "fn main() {println!(\"if you see this, we are rebuilding the dependencies of rumbas\")}" > rumbas/src/lib.rs
RUN cd rumbas && cargo build --target=x86_64-unknown-linux-musl --release
RUN rm -f rumbas/target/x86_64-unknown-linux-musl/release/deps/rumbas*

COPY rumbas/src rumbas/src
RUN cd rumbas && cargo build --target=x86_64-unknown-linux-musl --release

FROM alpine as numbas_fetcher
WORKDIR /usr/app
RUN apk add git
RUN git clone https://github.com/numbas/Numbas.git Numbas

WORKDIR /usr/app/Numbas
RUN git fetch && git checkout v6.0

# Fetch jsx graph extension
FROM alpine as jsxgraph_fetcher
WORKDIR /usr/app
RUN apk add git
RUN git clone https://github.com/numbas/numbas-extension-jsxgraph.git jsxgraph
WORKDIR /usr/app/jsxgraph
RUN git fetch && git checkout 9bc865f695009cf1942060be4e725e3dc687895b 

# Fetch stats extension
FROM alpine as stats_fetcher
WORKDIR /usr/app
RUN apk add git
RUN git clone https://github.com/numbas/numbas-extension-stats.git stats
WORKDIR /usr/app/stats
RUN git fetch && git checkout  62ed29f8ef06dafef7b9fc47dc843d668e119966

# Main image
FROM python:3.6.10-alpine 
WORKDIR /usr/app/Numbas

COPY --from=numbas_fetcher /usr/app/Numbas /usr/app/Numbas
RUN pip install -r requirements.txt

RUN mkdir -p extensions
COPY --from=jsxgraph_fetcher /usr/app/jsxgraph /usr/app/Numbas/extensions/jsxgraph
COPY --from=stats_fetcher /usr/app/stats /usr/app/Numbas/extensions/stats

ENV NUMBAS_FOLDER=/usr/app/Numbas

COPY --from=builder /usr/app/rumbas/target/x86_64-unknown-linux-musl/release/rumbas /bin/rumbas
WORKDIR /usr/app
COPY entrypoint.sh .
WORKDIR /rumbas
ENTRYPOINT ["/usr/app/entrypoint.sh"]
