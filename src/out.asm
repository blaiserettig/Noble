bits 64
default rel

segment .text
global mainCRTStartup

mainCRTStartup:
    mov eax, dword [i]
    mov dword [x], eax
    mov eax, dword [x]
    ret

segment .bss
x resd 1
