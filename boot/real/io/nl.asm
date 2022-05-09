INT_VIDEO equ 0x10

%macro scroll 0
call newline
%endmacro

newline:
pusha

mov ah, 0x0E
mov al, 0x0A ; newline char
int INT_VIDEO
mov al, 0x0D ; carriage return
int INT_VIDEO

popa
ret