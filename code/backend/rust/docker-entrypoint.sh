#!/bin/bash

./rust_crewwriter &
./rust_episodewriter &

exec "$@"
