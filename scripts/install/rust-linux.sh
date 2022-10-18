#!/bin/bash

curl https://sh.rustup.rs -sSf | sh -s -- -y
rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu
