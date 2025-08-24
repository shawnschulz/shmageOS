#!/bin/bash

# tests whether qemu can boot the os from UEFI
qemu-system-x86_64 -serial mon:stdio -vga std -bios /usr/share/qemu/OVMF.fd -cdrom build/os-x86_64.iso -m 512
