##
# shmageOS_x64
#
# @file
# @version 0.1

arch?=x86_64
kernel := build/kernel-$(arch).bin
iso := build/os-$(arch).iso
linker_script := src/arch/$(arch)/linker.ld
grub_cfg := src/arch/$(arch)/grub.cfg
assembly_source_files := $(wildcard src/arch/$(arch)/*.asm)
assembly_object_files := $(patsubst src/arch/$(arch)/%.asm, \
	build/arch/$(arch)/%.o, $(assembly_source_files))
target?=$(arch)-unknown-none
rust_os := target/$(target)/debug/libshos.a


.PHONY: all clean run iso kernel

all: $(kernel)
clean: @rm -r build
run: $(iso)
	@qemu-system-x86_64 -cdrom $(iso)

iso: $(iso)

$(iso): $(kernel) $(grub_cfg)
	@mkdir -p build/isofiles/boot/grub
	@cp $(kernel) build/isofiles/boot/kernel.bin
	@cp $(grub_cfg) build/isofiles/boot/grub
	@grub-mkstandalone -O x86_64-efi -o bootx64.efi "boot/grub/grub.cfg=iso_root/boot/grub/grub.cfg"
	@mv bootx64.efi build/isofiles/boot/
	@grub-mkrescue -d /usr/lib/grub/i386-pc -o $(iso) build/isofiles 2> /dev/null
# @rm -r build/isofiles

$(kernel): kernel $(rust_os) $(assembly_object_files) $(linker_script)
	@ld -n -T $(linker_script) -o $(kernel) \
		$(assembly_object_files) $(rust_os)

kernel:
	cargo build --target x86_64-unknown-none

# compile assembly files
build/arch/$(arch)/%.o: src/arch/$(arch)/%.asm
	@mkdir -p $(shell dirname $@)
	@nasm -felf64 $< -o $@

# end
