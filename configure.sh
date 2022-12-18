#!/bin/bash

set -x

mkdir -p build/
rm -rf isoroot/ # dir moved to build/
mkdir -p build/isoroot_x86_64/
mkdir -p build/isoroot_aarch64/
mkdir -p log/

