#!/usr/bin/env bash

# Building for Lambda requires either building with musl or building inside a Docker container
# See https://github.com/awslabs/aws-lambda-rust-runtime/issues/17
cargo build -p skill --target x86_64-unknown-linux-musl --release
cp ../target/x86_64-unknown-linux-musl/release/bootstrap bootstrap
strip bootstrap
zip -b /tmp rust.zip bootstrap
