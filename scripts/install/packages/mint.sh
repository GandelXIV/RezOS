#!/bin/bash
set -xe

# DEPENDENCIES:
# curl     - used to install rust
# make     - base build system, required
# nasm     - assembler, required
# gcc      - C compiler, used in BOOTBOOT and MAKEFILE2GRAPH, required if those are not already built
# aqemu    - QEMU emulator, required for debug/run
# graphviz - dot client, used to generate log/buildflow.png
# rustup   - the recomended rust toolchain installer/manager
# xorriso  - iso image maker 

apt install -y curl make nasm gcc aqemu graphviz mtools xorriso automake

