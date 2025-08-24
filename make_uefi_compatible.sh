#!/usr/bin/env sh
#
grub-mkstandalone \
   -O x86_64-efi \
   -o BOOTX64.EFI \
   "boot/grub/grub.cfg=iso_root/boot/grub/grub.cfg"

mkdir -p build/isofiles/efi/EFI/BOOT

mv BOOTX64.EFI build/isofiles/efi/EFI/BOOT/

grub-mkrescue -d /usr/lib/grub/x86_64-efi -o build/os-x86_64.iso build/isofiles
