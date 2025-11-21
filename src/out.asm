bits 64
default rel

segment .text
global mainCRTStartup

mainCRTStartup:
    mov dword [x], 0
    mov eax, 0
    mov dword [i], eax
loop_begin_i:
    mov eax, dword [i]
    mov ebx, 10
    cmp eax, ebx
    jg loop_end_i
    mov eax, dword [x]
    push rax
    mov ebx, dword [i]
    pop rax
    add eax, ebx
    mov dword [x], eax
    mov eax, dword [i]
    inc eax
    mov dword [i], eax
    jmp loop_begin_i
loop_end_i:
    mov dword [y], 0
    mov dword [z], 1078530000
    mov eax, dword [x]
    push rax
    mov ebx, 10
    pop rax
    add eax, ebx
    mov eax, eax
    push rax
    mov ebx, 5
    pop rax
    imul eax, ebx
    mov eax, eax
    push rax
    mov ebx, 2
    pop rax
    cdq
    idiv ebx
    mov dword [y], eax
    mov eax, dword [y]
    ret

segment .bss
z resd 1
y resd 1
i resd 1
x resd 1
