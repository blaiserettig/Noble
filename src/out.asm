bits 64
default rel

segment .text
global mainCRTStartup

mainCRTStartup:
    mov dword [x], 1
    ret

segment .bss
x resd 1
