#!/bin/bash

curl https://sh.rustup.rs -sSf | sh -s -- -y
source "$HOME/.cargo/env"
rustup toolchain install nightly
rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu

echo "Installed rust!"
echo "Dont forget to restart your current shell before building!"
