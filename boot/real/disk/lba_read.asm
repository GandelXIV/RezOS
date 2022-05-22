; DAPS = Disk Address Packet Structure
; db  size      -> size of the packet (16)
; db  void      -> always 0
; dw  secount   -> number of sectors to transfer (max 127 on some BIOSes)
; dd  buffer    -> transfer buffer (seg:off, write first off because x86 is lil-endian + align 16 bit
; dd  lower_lba -> lower 32 bits of starting LBA
; dd  upper_lba -> upper 16 bits of starting LBA

%macro rlba_read 2
mov ds:si, dword %2
mov ah, 0x42
mov dl, %1
int INT_DISK
%endmacro