#!/bin/bash

# Make a bootable usb by formatting the usb with a recognizable fs partition by GRUB, putting our
# kernel binary on there and doing anything else GRUB needs to boot into the OS

# install all this stuff if you haven't already
# sudo apt install git bison libopts25 libselinux1-dev m4 help2man libopts25-dev flex libfont-freetype-perl automake make autotools-dev autopoint libfreetype6-dev texinfo python autogen autoconf libtool libfuse3-3 unifont gettext binutils pkg-config liblzma5 libdevmapper-dev
# cd ~/Applications
# git clone git://git.savannah.gnu.org/grub.git
# cd grub
# ./bootstrap
#

# i.e. /dev/sda or whatever your usb is mounted to
USB_DISK_PATH=$1

sudo dd if=build/os-x86_64.iso of=$USB_DISK_PATH && sync
