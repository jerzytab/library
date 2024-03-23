FROM rust:latest as build

RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk wasm-bindgen-cli

