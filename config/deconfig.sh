#!/bin/bash

# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at https://mozilla.org/MPL/2.0/.

# This script cleans configuration links from the whole project

set -e

echo "Deconfiguring..."

set -x

rm -f kernel/kentry/aarch64/config.S
rm -f kernel/kentry/x86_64/config.asm
rm -f kernel/src/config.rs
rm -f config.mk

echo "Deconfigured!"
