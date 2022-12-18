#!/bin/bash

# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at https://mozilla.org/MPL/2.0/.

set -xe
# see more in scripts/install/packages/packages.md
pacman -Sy --noconfirm curl make nasm gcc qemu graphviz mtools xorriso automake clang llvm lld 

