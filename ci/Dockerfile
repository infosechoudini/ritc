FROM rust:alpine

ADD ./ ./

RUN apk update

RUN apk add gcc build-base musl-dev

RUN rustup toolchain list

RUN cargo bench