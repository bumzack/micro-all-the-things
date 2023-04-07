FROM ubuntu:latest AS builder

RUN apt-get update && apt install -y openssl  libssl-dev  &&  apt install  -y  gnupg ca-certificates curl

RUN curl -s https://repos.azul.com/azul-repo.key |   gpg --dearmor -o /usr/share/keyrings/azul.gpg
RUN echo "deb [signed-by=/usr/share/keyrings/azul.gpg] https://repos.azul.com/zulu/deb stable main" |   tee /etc/apt/sources.list.d/zulu.list
RUN apt update
RUN apt install -y zulu19-jdk
RUN apt install -y build-essential pkg-config

RUN curl https://sh.rustup.rs -sSf | bash -s -- -y

ENV PATH="/root/.cargo/bin:${PATH}"

COPY . .

RUN cargo build --release

FROM ubuntu:latest

RUN apt-get update && apt install -y openssl  libssl-dev  &&   apt install -y  gnupg ca-certificates curl
RUN curl -s https://repos.azul.com/azul-repo.key |   gpg --dearmor -o /usr/share/keyrings/azul.gpg
RUN echo "deb [signed-by=/usr/share/keyrings/azul.gpg] https://repos.azul.com/zulu/deb stable main" |  tee /etc/apt/sources.list.d/zulu.list
RUN apt update
RUN apt install -y zulu19-jdk

COPY --from=builder ./target/release/rust_crewwriter ./rust_crewwriter
COPY --from=builder ./target/release/rust_episodewriter ./rust_episodewriter

RUN find . -name "libssl.so*"

COPY ./docker-entrypoint.sh /

ENTRYPOINT ["/docker-entrypoint.sh"]

CMD ["ls"]

