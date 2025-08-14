bits 64
default rel

segment .text
global mainCRTStartup

mainCRTStartup:
    mov dword [x], 1
    mov eax, dword [x]
    mov dword [y], eax
    mov eax, dword [y]
    ret

segment .bss
y resd 1
x resd 1
