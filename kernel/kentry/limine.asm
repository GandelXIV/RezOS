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

; REQUESTS 

extern LIMINE_REQUEST_TERMINAL
extern LIMINE_REQUEST_BOOT_INFO
extern LIMINE_REQUEST_MEMORY_MAP

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

CALLBACK:
ret

LIMINE_REQUEST_MEMORY_MAP:
.common1  dq MAGIC_COMMON_A
.common2  dq MAGIC_COMMON_B
.feat1    dq MAGIC_MEMORY_MAP_A
.feat2    dq MAGIC_MEMORY_MAP_B
.revision dq 0
; pointer to the response
.response dq 0

