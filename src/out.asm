bits 64
default rel

segment .text
global mainCRTStartup

mainCRTStartup:
    mov dword [x], 0
    mov eax, dword [x]
    ret

segment .bss
x resd 1
