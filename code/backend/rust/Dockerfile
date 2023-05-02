# https://stackoverflow.com/questions/73871430/create-docker-image-from-rust-workspace


FROM ubuntu:latest AS builder

RUN apt-get update
RUN apt-get install -y \
    build-essential \
    curl \
    pkg-config \
    libssl-dev


RUN apt-get update

RUN curl https://sh.rustup.rs -sSf | bash -s -- -y

ENV PATH="/root/.cargo/bin:${PATH}"
RUN cargo --help

#RUN rustup target add x86_64-unknown-linux-musl
#RUN apt update && apt install -y musl-tools musl-dev && install -y pkg-config
#RUN update-ca-certificates

WORKDIR /usr/src/app

# copy entire workspace
COPY . .

EXPOSE 18320:18320
EXPOSE 18600:18600
EXPOSE 18200:18200

RUN cargo build   --release

FROM ubuntu:latest
COPY --from=builder /usr/src/app/target/release/rust_search_search_index ./
CMD [ "./rust_search_search_index" ]
LABEL service=rust_search_search_index

FROM ubuntu:latest
COPY --from=builder /usr/src/app/target/release/rust_search_article ./
CMD [ "./rust_search_article" ]
LABEL service=rust_search_article


FROM ubuntu:latest
COPY --from=builder /usr/src/app/target/release/rust_moviesearch ./
CMD [ "./rust_moviesearch" ]
LABEL service=rust_moviesearch
