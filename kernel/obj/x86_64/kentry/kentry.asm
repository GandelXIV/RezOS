; This Source Code Form is subject to the terms of the Mozilla Public
; License, v. 2.0. If a copy of the MPL was not distributed with this
; file, You can obtain one at https://mozilla.org/MPL/2.0/.

section .text

extern kmain
global _start

_start:
jmp kmain   ; main function linked from kernel src/main.rs
