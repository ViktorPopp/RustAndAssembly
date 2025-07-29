.intel_syntax noprefix

.section .data
    message: .asciz "Hello from Assembly!\n"

.section .text
.global print
print:
    mov rax, 1                  # Syscall number for write
    mov rdi, 1                  # File descriptor 1 (stdout)
    lea rsi, [rip + message]    # RIP-relative addressing for message
    mov rdx, 22                 # Length of the message (5 for "hello" + 1 for newline)
    syscall                     # Invoke the system call
    ret
