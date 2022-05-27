org 0x7C00  ; the offset BL is loaded from
bits 16     ; we start in real mode

entry:
jmp rmain  ; jumping to main so that we dont execute any includes (see below)

; ========================== INCLUDES

%include "boot/real/int.asm"
%include "boot/real/io/puts.asm"
%include "boot/real/io/putd.asm"
%include "boot/real/io/nl.asm"
%include "boot/real/abort.asm"

; ==========================  TEXT

; error handlers
on_debug:

rputsln MSG_DEBUG
rabort

on_lba_unsupported:

rputsln ERROR_LBA_ADDRESSING_UNSUPPORTED
rabort

; =========== STAGE @2

; _start equivalent
rmain:

mov [BOOT_DRIVE], dl  ; store current booted drive for later use

; cleanup registers
mov ax, 0
mov dx, ax
mov es, ax
mov ss, ax
mov sp, 0x7C00   ; setup stack

; print Init msg
rputsln MSG_INIT

; =========== STAGE @3

; TODO: load kernel

; =========== STAGE @4

; check for lower memory size and write it to MMAP_LOWER(from ax)
clc                     ; clear carry flag
int INT_LOWER_MEM_SIZE  ; request
jc .error     ; check for error
mov [MMAP_LOWER], ax    ; save size
jmp .continue

.error:
rputsln ERROR_CONVENTIONAL_MMAP_SIZE

.continue:
jmp $   ; halt
; ========================== DATA

; allocated
BOOT_DRIVE db 0
MMAP_LOWER db 0
MMAP_UPPER db 0 ; unsupported for now

; initialized
%include "boot/msg.asm"

SUPERBLOCK_LBA  equ 1
SUPERBLOCK_ALLOC equ 0x10000
SUPERBLOCK_DAPS:
    sizex     db 16
    void      db 0
    secount   dw 1
    buffer    dd SUPERBLOCK_ALLOC
    lba       dq SUPERBLOCK_LBA


; marks end of allocated data when looking at binary
; can be safely removed
._cookie db 0xFF
; ========================== padding and magic!

times 510-($-$$) db 0 ; fill the rest of the bootsector with nulls
dw 0xAA55   ; magic signature required to let BIOS know this disk is bootable
