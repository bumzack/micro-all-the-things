#!/bin/zsh

docker build -t rust_stuff    .
docker run   --cpus=".6" --name rust_stuff_service  rust_stuff


docker tag $(docker image ls --filter=dangling=true --filter=label=service=rust_search_search_index) rust_search_search_index
docker tag $(docker image ls --filter=dangling=true --filter=label=service=rust_search_article) rust_search_article
docker tag $(docker image ls --filter=dangling=true --filter=label=service=rust_moviesearch) rust_moviesearch


