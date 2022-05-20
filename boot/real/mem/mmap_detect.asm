%macro rmmap_detect_lower 0
clc                     ; clear carry flag
int INT_LOWER_MEM_SIZE  ; request
jc mmap_lower_error     ; check for error
mov [MMAP_LOWER], ax
%endmacro

mmap_lower_error:
rputsln ERROR_CONVENTIONAL_MMAP_SIZE
jmp $   ; halt