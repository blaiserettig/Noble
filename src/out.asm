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
    ret

segment .bss
i resd 1
x resd 1
