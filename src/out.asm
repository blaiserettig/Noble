bits 64
default rel

segment .text
global mainCRTStartup

mainCRTStartup:
    mov eax, 1
    ret