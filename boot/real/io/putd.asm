N_TO_ASCII_DIFF equ 48  ; int(n + this = 

%macro rputd 1
pusha
mov ax, %1
call rputd_start
popa
%endmacro

; args: ax=num
rputd_start:
mov dx, 0       ; rest
mov bx, RPUTD_SELECTORS  ; digit selector

rputd_loop:
div word [bx]          ; div ax bx => ax=digit, dx=rest
add ax, N_TO_ASCII_DIFF
mov ah, 0x0E
int INT_VIDEO
mov ax, dx
cmp bx, RPUTD_SELECTORS + 4
je rputd_end
inc bx
jmp rputd_loop

rputd_end:
ret


; immutable data

RPUTD_SELECTORS dw 10000, 01000, 00100, 00010, 000001