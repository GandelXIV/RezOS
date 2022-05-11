org 0x7C00  ; the offset BL is loaded from
bits 16     ; we start in real mode

entry:
jmp real_main  ; jumping to main so that we dont execute any includes

; ========================== INCLUDES

%include "boot/real/io/puts.asm"
%include "boot/real/io/nl.asm"

; ==========================  TEXT
real_main:

; ===== STAGE 1
mov [BOOT_DRIVE], dl  ; store for later use

; cleanup registers
mov ax, 0
mov dx, ax
mov es, ax
mov ss, ax
mov sp, 7C00h   ; setup stack

real_putsln MSG_INIT

jmp $   ; halt

; ========================== DATA
BOOT_DRIVE db 0
%include "boot/msg.asm"

times 510-($-$$) db 0 ; fill the rest of the bootsector with nulls
dw 0AA55h   ; magic signature required to let BIOS know this disk is bootable
