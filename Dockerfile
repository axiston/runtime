# Compile & build the application.
FROM rust:latest AS build
WORKDIR /usr/src/runtime/

# Configurate & run the application.
FROM debian:buster-slim AS run
WORKDIR /usr/bin/runtime/
