#!/bin/bash
set -xe

# DEPENDENCIES:
# curl     - used to install rust
# make     - base build system, required
# nasm     - assembler, required
# gcc      - C compiler, used in BOOTBOOT and MAKEFILE2GRAPH, required if those are not already built
# qemu     - QEMU emulator, required for debug/run
# graphviz - dot client, used to generate log/buildflow.png
# xorriso  - iso image maker 

pacman -Sy --noconfirm curl make nasm gcc qemu graphviz mtools xorriso

