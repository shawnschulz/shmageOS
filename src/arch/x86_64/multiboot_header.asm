;; This file indicates multiboot 2 support to GRUB2

section .multiboot_header
    header_start:
        dd 0xe85250d6           ;32 bit magic number
        dd 0                    ;architecture 0
        dd header_end - header_start
        dd 0x100000000 - (0xe85250d6 + 0 + (header_end - header_start))
    ;; optional multiboot tags should go here

    ;; required end tags
        dw 0                    ;type
        dw 0                    ;flags
        dd 8                    ;size
    header_end:
