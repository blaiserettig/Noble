bits 64
default rel

segment .text
global mainCRTStartup

mainCRTStartup:
    mov eax, dword [i]
    mov dword [x], eax
    mov eax, 0
    mov dword [i], eax
loop_begin_i:
    mov eax, dword [i]
    mov ebx, 10
    cmp eax, ebx
    jg loop_end_i
    mov eax, dword [i]
    mov dword [x], eax
    mov eax, dword [i]
    inc eax
    mov dword [i], eax
    jmp loop_begin_i
loop_end_i:
    mov eax, dword [x]
    mov dword [y], eax
    mov eax, dword [y]
    ret

segment .bss
x resd 1
y resd 1
i resd 1
