#!/bin/bash

# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at https://mozilla.org/MPL/2.0/.

# Run this script only from the project root!
# Spins-up a container from our previously built image (in this case named 'rezenv'),
# with an interface, and mounts the working directory in /home/rezos/
docker run -v $(pwd):/home/rezos -it rezenv
