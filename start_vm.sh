#!/bin/bash

# Use the phil-opp.com bootimage creator (makes an ELF file that links
# a bootloader to your kernel
cargo bootimage

# Run the boot image
qemu-system-x86_64 -drive format=raw,file=/home/bankerz/Programs/shos/target/x86_64-shos/debug/bootimage-shos.bin
