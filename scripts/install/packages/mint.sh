#!/bin/bash
set -xe
# see more in scripts/install/packages/packages.md
apt update
apt install -y curl make nasm gcc aqemu graphviz mtools xorriso automake

