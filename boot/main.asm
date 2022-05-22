org 0x7C00  ; the offset BL is loaded from
bits 16     ; we start in real mode

entry:
jmp rmain  ; jumping to main so that we dont execute any includes (see below)

; ========================== INCLUDES

%include "boot/real/int.asm"
%include "boot/real/io/puts.asm"
%include "boot/real/io/nl.asm"

; ==========================  TEXT

on_lba_unsupported:

rputsln PANIC_LBA_ADDRESSING_UNSUPPORTED
jmp $   ; halt

rmain:

; ===== STAGE 1
mov [BOOT_DRIVE], dl  ; store current booted drive for later use

; cleanup registers
mov ax, 0
mov dx, ax
mov es, ax
mov ss, ax
mov sp, 0x7C00   ; setup stack

; print Init msg
rputsln MSG_INIT

; check for lower memory size and write it to MMAP_LOWER(from ax)
clc                     ; clear carry flag
int INT_LOWER_MEM_SIZE  ; request
jc .error     ; check for error
mov [MMAP_LOWER], ax    ; save size
jmp .continue

.error:
rputsln ERROR_CONVENTIONAL_MMAP_SIZE

.continue:

; check for LBA addressing
clc
mov ah, 0x41
mov bx, 0x55AA
mov dl, 0x80
int 0x13
jc on_lba_unsupported

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
dw 0xAA55   ; magic signature required to let BIOS know this disk is bootable
