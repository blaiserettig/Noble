bits 64
default rel

segment .text
global mainCRTStartup

mainCRTStartup:
    mov eax, 10
    ret