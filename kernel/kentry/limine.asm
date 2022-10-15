; LIMINE REQUESTS 

extern LIMINE_REQUEST_TERMINAL
extern LIMINE_REQUEST_BOOT_INFO

LIMINE_REQUEST_BOOT_INFO:
; request ID
.common1  dq 0xc7b1dd30df4c8b88
.common2  dq 0x0a82e883a194f07b
.info1    dq 0xf55038d8e2a1202f
.info2    dq 0x279426fcf5f59740
.revision dq 0
; pointer to the response
.response dq 0

LIMINE_REQUEST_TERMINAL:
; request ID
.common1  dq 0xc7b1dd30df4c8b88
.common2  dq 0x0a82e883a194f07b
.info1    dq 0xc8ac59310c2b0844
.info2    dq 0xa68d0c7265d38878
.revision dq 0
.response dq 0
.callback dq CALLBACK

CALLBACK:
ret

