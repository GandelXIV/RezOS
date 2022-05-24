N_TO_ASCII_DIFF equ 48  ; int(n + this = 

; args: 16bit number
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
div word [bx]           ; div ax bx => ax=digit, dx=rest
add ax, N_TO_ASCII_DIFF ; convert int to char
mov ah, 0x0E            ; print config
int INT_VIDEO
mov ax, dx              ; move rest to ax so it can be reused in the next iteration
mov dx, 0               ; reset dx
cmp bx, RPUTD_SELECTORS + 4 ; check if all selectors have been used
je rputd_end
inc bx
jmp rputd_loop

rputd_end:
ret


; immutable data

RPUTD_SELECTORS dw 10000, 01000, 00100, 00010, 000001
