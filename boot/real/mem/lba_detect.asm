; arg: address to jump to if unsupported
%macro lba_detect 1
clc
mov ah, 0x41
mov bx, 0x55AA
mov dl, 0x80
int 0x13
jc %1
%endmacro