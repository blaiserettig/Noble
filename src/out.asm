bits 64
default rel

segment .text
global mainCRTStartup

mainCRTStartup:
    mov dword [x], 0
    mov eax, dword [x]
    push rax
    mov ebx, 0
    pop rax
    cmp eax, ebx
    sete al
    movzx eax, al
    mov eax, eax
    cmp eax, 0
    je endif_0
    mov dword [x], 1
endif_0:
    mov eax, dword [x]
    ret

segment .bss
x resd 1
