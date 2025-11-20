bits 64
default rel

segment .text
global mainCRTStartup

mainCRTStartup:
    mov dword [x], 5
    mov eax, dword [x]
    push rax
    mov ebx, 5
    pop rax
    add eax, ebx
    mov eax, eax
    push rax
    mov ebx, 2
    pop rax
    imul eax, ebx
    mov dword [x], eax
    mov eax, dword [x]
    ret

segment .bss
x resd 1
