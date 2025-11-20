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
    mov dword [z], 1068876431
    mov dword [b], 1
    mov eax, dword [y]
    ret

segment .bss
i resd 1
y resd 1
z resd 1
x resd 1
b resd 1
