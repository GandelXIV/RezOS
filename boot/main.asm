org 0x7C00  ; the offset BL is loaded from
bits 16     ; we start in real mode

entry:
jmp rmain  ; jumping to main so that we dont execute any includes (see below)

; ========================== INCLUDES

%include "boot/real/int.asm"
%include "boot/real/io/puts.asm"
%include "boot/real/io/nl.asm"
%include "boot/real/mem/mmap_detect.asm"
%include "boot/real/mem/lba_detect.asm"

; ==========================  TEXT

on_lba_unsupported:
rputsln PANIC_LBA_ADDRESSING_UNSUPPORTED
jmp $

rmain:

; ===== STAGE 1
mov [BOOT_DRIVE], dl  ; store current booted drive for later use

; cleanup registers
mov ax, 0
mov dx, ax
mov es, ax
mov ss, ax
mov sp, 0x7C00   ; setup stack

rputsln MSG_INIT

; check for lower memory size and write it to MMAP_LOWER(from ax)
rmmap_detect_lower
; check for LBA addressing
lba_detect on_lba_unsupported

jmp $   ; halt
; ========================== DATA

; initialized
%include "boot/msg.asm"
; allocated
BOOT_DRIVE db 0
MMAP_LOWER db 0
MMAP_UPPER db 0 ; unsupported for now

; ========================== padding and magic!

times 510-($-$$) db 0 ; fill the rest of the bootsector with nulls
dw 0AA55h   ; magic signature required to let BIOS know this disk is bootable
