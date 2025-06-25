#!/bin/bash

# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at https://mozilla.org/MPL/2.0/.

set -xe
# see more in scripts/install/packages/packages.md
apt update
apt install -y curl make nasm gcc aqemu graphviz mtools xorriso automake llvm lld gcc-aarch64-linux-gnu git clang
