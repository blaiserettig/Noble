bits 64
default rel

segment .text
global mainCRTStartup

mainCRTStartup:
    mov eax, 0
    ret