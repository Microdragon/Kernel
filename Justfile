# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.
bootloader := "limine"
target := "x86_64-unknown-none"
release := "false"

qemu_command := if target == "x86_64-unknown-none" {
  "qemu-system-x86_64 -cpu qemu64"
} else { "" }

# opens a chooser of possible targets
default:
  just --list

# Builds the kernel for the given target and bootloader
build:
  @echo Building microdragon with {{ bootloader }} Bootloader for {{ target }} target
  cargo build -p microdragon-{{ bootloader }} --target {{ target }} {{ if release =~ "true|1|yes" { "--release" } else { "" } }}

# Packs the kernel into an iso file, bootable using UEFI and legacy BIOS boot
@pack: _check_xorriso build
  -[ -d disk ] && rm -rf disk
  mkdir -p disk/system
  @cp target/{{ target }}/{{ if release =~ "true|1|yes" { "release" } else { "debug" } }}/microdragon-{{ bootloader }} disk/system/kernel
  just _pack_{{ bootloader }}

# Runs the kernel in QEMU using legacy BIOS boot
run_bios: pack
  {{ qemu_command }} -cdrom microdragon.iso

# Runs the kernel in QEMU using UEFI boot (WIP)
run_uefi: pack
  {{ qemu_command }} -bios deps/OVMF/OVMF.fd -cdrom microdragon.iso

license:
  #!/bin/sh
  TEST='// This Source Code Form is subject to the terms of the Mozilla Public'
  NOTICE="// This Source Code Form is subject to the terms of the Mozilla Public\n// License, v. 2.0. If a copy of the MPL was not distributed with this\n// file, You can obtain one at http://mozilla.org/MPL/2.0/."
  FILES=`find . -type f -name '*.rs' -not -path './libs/*'`
  for file in $FILES; do
    if [ "$(head -n 1 $file)" != "$TEST" ]; then
      echo -E "$NOTICE\n$(cat $file)" > $file
      echo Updated license header for $file
    fi
  done

_pack_limine: _install_limine
  @echo Copying bootloader files...
  @mkdir -p disk/limine
  cp deps/limine/limine-uefi-cd.bin deps/limine/limine-bios-cd.bin deps/limine/limine-bios.sys disk/limine
  cp bootloader/limine/limine.cfg disk/limine
  @mkdir -p disk/EFI/BOOT
  cp deps/limine/BOOTAA64.EFI deps/limine/BOOTRISCV64.EFI deps/limine/BOOTX64.EFI disk/EFI/BOOT

  @echo Creating ISO image
  xorriso -as mkisofs -b limine/limine-bios-cd.bin \
          -no-emul-boot -boot-load-size 4 -boot-info-table \
          --efi-boot limine/limine-uefi-cd.bin \
          -efi-boot-part --efi-boot-image --protective-msdos-label \
          disk -o microdragon.iso
  {{ if os() == "windows" { "./deps/limine/limine.exe" } else { "./deps/limine/limine" } }} bios-install microdragon.iso

_install_limine:
  #!/bin/sh
  if [ ! -d 'deps/limine' ]
  then
    echo Downloading limine binaries...
    set -x
    git clone https://github.com/limine-bootloader/limine deps/limine -b v5.x-branch-binary --depth 1
    rm -rf deps/limine/.git
    {{ if os() != "windows" { "cd deps/limine && make all" } else { "" } }}
  fi

_check_xorriso:
  #!/bin/sh
  if ! command -v xorriso &> /dev/null
  then
    echo Please install xorriso using your package manager.
    exit 1
  fi
