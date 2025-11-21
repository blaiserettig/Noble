bits 64
default rel

segment .text
global mainCRTStartup

mainCRTStartup:
    mov dword [c], 97
    mov dword [d], 98
    mov eax, dword [c]
    push rax
    mov ebx, dword [d]
    pop rax
    cmp eax, ebx
    setl al
    movzx eax, al
    mov eax, eax
    cmp eax, 0
    je else_0
    mov eax, 1
    jmp endif_0
else_0:
    mov eax, 0
endif_0:
    ret

segment .bss
c resd 1
d resd 1
