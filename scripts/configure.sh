#!/bin/bash

# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at https://mozilla.org/MPL/2.0/.

set -x

mkdir -p build/
rm -rf isoroot/ # dir moved to build/
mkdir -p build/isoroot_x86_64/
mkdir -p build/isoroot_aarch64/
mkdir -p log/
