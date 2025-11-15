bits 64
default rel

segment .text
global mainCRTStartup

mainCRTStartup:
    mov dword [x], 1
    mov dword [y], 10
    mov dword [z], 15
    mov dword [x], 1
    mov eax, dword [x]
    ret

segment .bss
x resd 1
z resd 1
y resd 1
