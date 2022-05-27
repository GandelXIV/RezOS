N_TO_ASCII_DIFF equ 48  ; int(n) + this = char(n)

; args: 16bit numberto put on screen
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
div word [bx]           ; div ax/bx => ax=digit, dx=rest
add ax, N_TO_ASCII_DIFF ; convert int to char
mov ah, 0x0E            ; putchar config
int INT_VIDEO           ; call print driver
mov ax, dx              ; move rest to ax so it can be reused in the next iteration
mov dx, 0               ; reset dx
; if all selectors have been used, jump to the end
cmp bx, RPUTD_SELECTORS + 8
je rputd_end
; advance to next selector
inc bx
inc bx
; repeat
jmp rputd_loop

rputd_end:
ret


; immutable data

RPUTD_SELECTORS dw 10000, 1000, 100, 10, 1  ; used to select specific digits
