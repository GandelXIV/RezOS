; CHS = Cylinders, Heads, Sectors

; Gets Sectors/Track, Head count
; args: 1=DRIVE 2='ptr to set head count' 3='ptr to set sectors/track' 
%macro chs_get 3
mov ah, 8
mov dl, %1
int INT_DISK
; get head count
sub dh, 1
mov [%2], dh
; get sectors/track
and cl, 0x3F
mov [%3], cl
%endmacro
