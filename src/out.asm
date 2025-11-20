bits 64
default rel

segment .text
global mainCRTStartup

mainCRTStartup:
    mov dword [x], 5
