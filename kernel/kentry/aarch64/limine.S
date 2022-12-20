/* This Source Code Form is subject to the terms of the Mozilla Public
 License, v. 2.0. If a copy of the MPL was not distributed with this
 file, You can obtain one at https://mozilla.org/MPL/2.0/. 
 */

// This file holds all the limine requests that then get detected by the bootloader

.equ MAGIC_COMMON_A, 0xc7b1dd30df4c8b88
.equ MAGIC_COMMON_B, 0x0a82e883a194f07b
.equ MAGIC_BOOT_INFO_A, 0xf55038d8e2a1202f
.equ MAGIC_BOOT_INFO_B, 0x279426fcf5f59740
.equ MAGIC_TERMINAL_A, 0xc8ac59310c2b0844
.equ MAGIC_TERMINAL_B, 0xa68d0c7265d38878
.equ MAGIC_MEMORY_MAP_A, 0x67cf3d9d378a806f
.equ MAGIC_MEMORY_MAP_B, 0xe304acdfc50c3c62
.equ MAGIC_BOOT_TIME_A, 0x502746e184c088aa
.equ MAGIC_BOOT_TIME_B, 0xfbc5ec83e6327893
.equ MAGIC_KERNEL_ADRESS_A, 0x71ba76863cc55f63 
.equ MAGIC_KERNEL_ADRESS_B, 0xb2644a48c516a487
.equ MAGIC_HHDM_A, 0x48dcf1cb8ad2b852
.equ MAGIC_HHDM_B, 0x63984e959a98244b
.equ MAGIC_STACK_SIZE_A, 0x224ef0460a8e8926
.equ MAGIC_STACK_SIZE_B, 0xe1cb0fc25f46ea3d


.globl LIMINE_REQUEST_TERMINAL
.globl LIMINE_REQUEST_BOOT_INFO
.globl LIMINE_REQUEST_MEMORY_MAP
.globl LIMINE_REQUEST_BOOT_TIME
.globl LIMINE_REQUEST_KERNEL_ADDRESS
.globl LIMINE_REQUEST_HHDM
.globl LIMINE_REQUEST_STACK_SIZE

LIMINE_REQUEST_BOOT_INFO:
/* common magic */
.quad MAGIC_COMMON_A
.quad MAGIC_COMMON_B
/* feature specific magic */
.quad MAGIC_BOOT_INFO_A
.quad MAGIC_BOOT_INFO_B
.quad 0 // revision
.quad 0 // ptr to response

LIMINE_REQUEST_TERMINAL:
/* common magic */
.quad MAGIC_COMMON_A
.quad MAGIC_COMMON_B
/* feature specific magic */
.quad MAGIC_TERMINAL_A
.quad MAGIC_TERMINAL_B
.quad 0 // revision
.quad 0 // ptr to response
.quad callback // special ptr to callback handler, set by the kernel but we put in a placeholder

LIMINE_REQUEST_MEMORY_MAP:
/* common magic */
.quad MAGIC_COMMON_A
.quad MAGIC_COMMON_B
/* feature specific magic */
.quad MAGIC_MEMORY_MAP_A
.quad MAGIC_MEMORY_MAP_B
.quad 0 // revision
.quad 0 // ptr to response

LIMINE_REQUEST_BOOT_TIME:
/* common magic */
.quad MAGIC_COMMON_A
.quad MAGIC_COMMON_B
/* feature specific magic */
.quad MAGIC_BOOT_TIME_A
.quad MAGIC_BOOT_TIME_B
.quad 0 // revision
.quad 0 // ptr to response

LIMINE_REQUEST_KERNEL_ADDRESS:
/* common magic */
.quad MAGIC_COMMON_A
.quad MAGIC_COMMON_B
/* feature specific magic */
.quad MAGIC_KERNEL_ADRESS_A
.quad MAGIC_KERNEL_ADRESS_B
.quad 0 // revision
.quad 0 // ptr to response

LIMINE_REQUEST_HHDM:
/* common magic */
.quad MAGIC_COMMON_A
.quad MAGIC_COMMON_B
/* feature specific magic */
.quad MAGIC_HHDM_A
.quad MAGIC_HHDM_B
.quad 0 // revision
.quad 0 // ptr to response

LIMINE_REQUEST_STACK_SIZE:
/* common magic */
.quad MAGIC_COMMON_A
.quad MAGIC_COMMON_B
/* feature specific magic */
.quad MAGIC_HHDM_A
.quad MAGIC_HHDM_B
.quad 0 // revision
.quad 0 // ptr to response
// requested stack size, 16MiB to make sure
.quad 0xFFFFFF

callback:
