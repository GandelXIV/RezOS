; This Source Code Form is subject to the terms of the Mozilla Public
; License, v. 2.0. If a copy of the MPL was not distributed with this
; file, You can obtain one at https://mozilla.org/MPL/2.0/.

; This file holds all the limine requests that then get detected by the bootloader

; CONSTANTS
MAGIC_COMMON_A equ 0xc7b1dd30df4c8b88
MAGIC_COMMON_B equ 0x0a82e883a194f07b
MAGIC_BOOT_INFO_A equ 0xf55038d8e2a1202f
MAGIC_BOOT_INFO_B equ 0x279426fcf5f59740
MAGIC_TERMINAL_A equ 0xc8ac59310c2b0844
MAGIC_TERMINAL_B equ 0xa68d0c7265d38878
MAGIC_MEMORY_MAP_A equ 0x67cf3d9d378a806f
MAGIC_MEMORY_MAP_B equ 0xe304acdfc50c3c62
MAGIC_BOOT_TIME_A equ 0x502746e184c088aa
MAGIC_BOOT_TIME_B equ 0xfbc5ec83e6327893
MAGIC_KERNEL_ADRESS_A equ 0x71ba76863cc55f63 
MAGIC_KERNEL_ADRESS_B equ 0xb2644a48c516a487
MAGIC_HHDM_A equ 0x48dcf1cb8ad2b852
MAGIC_HHDM_B equ 0x63984e959a98244b
MAGIC_STACK_SIZE_A equ 0x224ef0460a8e8926
MAGIC_STACK_SIZE_B equ 0xe1cb0fc25f46ea3d

; REQUESTS 

extern LIMINE_REQUEST_TERMINAL
extern LIMINE_REQUEST_BOOT_INFO
extern LIMINE_REQUEST_MEMORY_MAP
extern LIMINE_REQUEST_BOOT_TIME
extern LIMINE_REQUEST_KERNEL_ADDRESS
extern LIMINE_REQUEST_HHDM
extern LIMINE_REQUEST_STACK_SIZE


LIMINE_REQUEST_BOOT_INFO:
.common1  dq MAGIC_COMMON_A
.common2  dq MAGIC_COMMON_B
.feat1    dq MAGIC_BOOT_INFO_A
.feat2    dq MAGIC_BOOT_INFO_B
.revision dq 0 
; pointer to the response
.response dq 0

LIMINE_REQUEST_TERMINAL:
.common1  dq MAGIC_COMMON_A
.common2  dq MAGIC_COMMON_B
.feat1    dq MAGIC_TERMINAL_A
.feat2    dq MAGIC_TERMINAL_B
.revision dq 0
; pointer to the response
.response dq 0
; used by the write() function returned by this feature
; we provide a default implementation CALLBACK
.callback dq CALLBACK

LIMINE_REQUEST_MEMORY_MAP:
.common1  dq MAGIC_COMMON_A
.common2  dq MAGIC_COMMON_B
.feat1    dq MAGIC_MEMORY_MAP_A
.feat2    dq MAGIC_MEMORY_MAP_B
.revision dq 0
; pointer to the response
.response dq 0

LIMINE_REQUEST_BOOT_TIME:
.common1  dq MAGIC_COMMON_A
.common2  dq MAGIC_COMMON_B
.feat1    dq MAGIC_BOOT_TIME_A
.feat2    dq MAGIC_BOOT_TIME_B
.revision dq 0
; pointer to the response
.response dq 0

LIMINE_REQUEST_KERNEL_ADDRESS:
.common1  dq MAGIC_COMMON_A
.common2  dq MAGIC_COMMON_B
.feat1    dq MAGIC_KERNEL_ADRESS_A 
.feat2    dq MAGIC_KERNEL_ADRESS_B
.revision dq 0
; pointer to the response
.response dq 0

LIMINE_REQUEST_HHDM:
.common1  dq MAGIC_COMMON_A
.common2  dq MAGIC_COMMON_B
.feat1    dq MAGIC_HHDM_A
.feat2    dq MAGIC_HHDM_B
.revision dq 0
; pointer to the response
.response dq 0

LIMINE_REQUEST_STACK_SIZE:
.common1  dq MAGIC_COMMON_A
.common2  dq MAGIC_COMMON_B
.feat1    dq MAGIC_STACK_SIZE_A
.feat2    dq MAGIC_STACK_SIZE_B
.revision dq 0
; pointer to the response
.response dq 0
; requested stack size
; 16 MiB to make sure
.size     dq 0xFFFFFF

; keep this on the bottom
CALLBACK:

