#!/bin/bash

# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at https://mozilla.org/MPL/2.0/.

# This script links config files from a provided profile

set -e

PROFILE_DIR="config/profiles/"

if [ $# -eq 0 ]
  then
    echo "No profile supplied!"
    echo "Profile list:"
    ls $PROFILE_DIR
    exit
fi

PROFILE=$PROFILE_DIR/$1

echo "Creating directories"

mkdir -p build/
mkdir -p build/isoroot_aarch64/
mkdir -p build/isoroot_x86_64/
mkdir -p log/

echo "Configuring..."

set -x

#ln $PROFILE/kentry-aarch64.S   kernel/kentry/aarch64/config.S
ln $PROFILE/limfeats-config-x86_64.asm  kernel/obj/x86_64/limfeats/config.asm
ln $PROFILE/rkernel.rs         kernel/src/config.rs
ln $PROFILE/Makeconfig.mk      config.mk

echo "Configured!"
