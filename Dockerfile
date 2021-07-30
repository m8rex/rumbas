FROM rust:1.53.0-slim as builder

WORKDIR /usr/app
RUN rustup target add x86_64-unknown-linux-musl

#use tricks of https://shaneutt.com/blog/rust-fast-small-docker-image-builds/
RUN mkdir numbas
COPY numbas/Cargo* numbas/
COPY numbas/src numbas/src

RUN mkdir -p rumbas/src
COPY rumbas/Cargo* rumbas/
COPY rumbas/src/lib.rs rumbas/src/lib.rs
RUN echo "fn main() {println!(\"if you see this, we are rebuilding the dependencies of rumbas\")}" > rumbas/src/data.rs
RUN echo "fn main() {println!(\"if you see this, we are rebuilding the dependencies of rumbas\")}" > rumbas/src/main.rs
RUN cd rumbas && cargo build --target=x86_64-unknown-linux-musl --release
RUN rm -f rumbas/target/x86_64-unknown-linux-musl/release/deps/rumbas*
RUN rm -f rumbas/src/data.rs

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
RUN git fetch && git checkout 62ed29f8ef06dafef7b9fc47dc843d668e119966

# Fetch euklides extension
FROM alpine as eukleides_fetcher
WORKDIR /usr/app
RUN apk add git
RUN git clone https://github.com/numbas/numbas-extension-eukleides.git eukleides
WORKDIR /usr/app/eukleides
RUN git fetch && git checkout bac3d060cd70d79fb6f897f0a54076ec916b8e14

# Fetch geogebra extension
FROM alpine as geogebra_fetcher
WORKDIR /usr/app
RUN apk add git
RUN git clone https://github.com/numbas/numbas-extension-geogebra.git geogebra
WORKDIR /usr/app/geogebra
RUN git fetch && git checkout 14fdb023341357134b6376f5f6084834587d6f8f

# Fetch random_person extension
FROM alpine as random_person_fetcher
WORKDIR /usr/app
RUN apk add git
RUN git clone https://github.com/numbas/numbas-extension-random-person.git random_person
WORKDIR /usr/app/random_person
RUN git fetch && git checkout 4031704f9570b0ad8a6918a0dc1e8063220392a8

# Fetch download_text_file extension
FROM alpine as download_text_file_fetcher
WORKDIR /usr/app
RUN apk add git
RUN git clone https://github.com/numbas/numbas-extension-download-a-text-file.git download-text-file
WORKDIR /usr/app/download-text-file
RUN git fetch && git checkout 32b99089a6d9837565a183e70f13d6351db61782

# Fetch codewords (linear codes) extension
FROM alpine as codewords_fetcher
WORKDIR /usr/app
RUN apk add git
RUN git clone https://github.com/numbas/numbas-extension-codewords.git codewords
WORKDIR /usr/app/codewords
RUN git fetch && git checkout 24b82c6d57027d33fffb8a58493174743d202d41

# Fetch permutations extension
FROM alpine as permutations_fetcher
WORKDIR /usr/app
RUN apk add git
RUN git clone https://github.com/numbas/numbas-extension-permutations.git permutations
WORKDIR /usr/app/permutations
RUN git fetch && git checkout 9b6b7a44c6b7dcbf03b1a7ffd03ed383194da721

# Fetch quantities extension
FROM alpine as quantities_fetcher
WORKDIR /usr/app
RUN apk add git
RUN git clone https://github.com/numbas/numbas-extension-quantities.git quantities
WORKDIR /usr/app/quantities
RUN git fetch && git checkout 05fa4bba4bbac078747c6dab2600496036a82857

# Fetch optimisation extension
FROM alpine as optimisation_fetcher
WORKDIR /usr/app
RUN apk add git
RUN git clone https://github.com/numbas/numbas-extension-optimisation.git optimisation
WORKDIR /usr/app/optimisation
RUN git fetch && git checkout 06899711367414950c7118329cb7c7c1bbb0542e

# Fetch polynomials extension
FROM alpine as polynomials_fetcher
WORKDIR /usr/app
RUN apk add git
RUN git clone https://github.com/numbas/numbas-extension-polynomials.git polynomials
WORKDIR /usr/app/polynomials
RUN git fetch && git checkout ab321aa13dc80609393553190233d1a771d04e7c

# Fetch chemistry extension
FROM alpine as chemistry_fetcher
WORKDIR /usr/app
RUN apk add git
RUN git clone https://github.com/numbas/numbas-extension-chemistry.git chemistry
WORKDIR /usr/app/chemistry
RUN git fetch && git checkout 6527a4690bd7ee5bca5e4f54facd8170eb018a2e

# Main image
FROM python:3.6.10-alpine 
WORKDIR /usr/app/Numbas

COPY --from=numbas_fetcher /usr/app/Numbas /usr/app/Numbas
RUN pip install -r requirements.txt

RUN mkdir -p extensions
COPY --from=jsxgraph_fetcher /usr/app/jsxgraph /usr/app/Numbas/extensions/jsxgraph
COPY --from=stats_fetcher /usr/app/stats /usr/app/Numbas/extensions/stats
COPY --from=geogebra_fetcher /usr/app/geogebra /usr/app/Numbas/extensions/geogebra
COPY --from=random_person_fetcher /usr/app/random_person /usr/app/Numbas/extensions/random_person
COPY --from=download_text_file_fetcher /usr/app/download-text-file /usr/app/Numbas/extensions/download-text-file
COPY --from=codewords_fetcher /usr/app/codewords /usr/app/Numbas/extensions/codewords
COPY --from=permutations_fetcher /usr/app/permutations /usr/app/Numbas/extensions/permutations
COPY --from=quantities_fetcher /usr/app/quantities /usr/app/Numbas/extensions/quantities
COPY --from=optimisation_fetcher /usr/app/optimisation /usr/app/Numbas/extensions/optimisation
COPY --from=polynomials_fetcher /usr/app/polynomials /usr/app/Numbas/extensions/polynomials
COPY --from=chemistry_fetcher /usr/app/chemistry /usr/app/Numbas/extensions/chemistry
# From git? Repo not found
COPY extensions/written_number /usr/app/Numbas/extensions/written-number
COPY extensions/graphs /usr/app/Numbas/extensions/graphs
RUN mkdir -p extensions/eukleides
 # For now just use the js file in dist instead of using make
COPY --from=eukleides_fetcher /usr/app/eukleides/dist/eukleides.js /usr/app/Numbas/extensions/eukleides
ENV NUMBAS_FOLDER=/usr/app/Numbas

COPY --from=builder /usr/app/rumbas/target/x86_64-unknown-linux-musl/release/rumbas /bin/rumbas
WORKDIR /usr/app
COPY entrypoint.sh .
WORKDIR /rumbas
ENTRYPOINT ["/usr/app/entrypoint.sh"]
