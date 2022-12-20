/* This Source Code Form is subject to the terms of the Mozilla Public
 License, v. 2.0. If a copy of the MPL was not distributed with this
 file, You can obtain one at https://mozilla.org/MPL/2.0/. 
 */

.text

.extern kmain
.globl _start

.include "kernel/kentry/aarch64/limine.S"

_start:
  b kmain
