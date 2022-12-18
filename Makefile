# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at https://mozilla.org/MPL/2.0/.

############ OPTIONS

# either yes|empty
KERNEL_BUILD_RELEASE ?=

# cargo command
CARGO ?= cargo

# path to make2graph executable
PATH_MAKEFILE2GRAPH ?= makefile2graph/make2graph

# path to limine generated binaries
PATH_LIMINE_BIN ?= limine/bin/

############ GENERICS

RKERNEL_SRC = kernel/Cargo* \
							kernel/kernel.ld \
							kernel/rust-toolchain \
							kernel/src/* \
							kernel/src/memman/* \
							kernel/src/arch/* \
							kernel/.cargo/* \

# https://stackoverflow.com/questions/2483182/recursive-wildcards-in-gnu-make
rwildcard=$(foreach d,$(wildcard $(1:=/*)),$(call rwildcard,$d,$2) $(filter $(subst *,%,$2),$d))

# 1 = isoroot dir
# 2 = output filename
define generate_iso_image_base
	xorriso -as mkisofs -b limine-cd.bin -no-emul-boot -boot-load-size 4 -boot-info-table --efi-boot limine-cd-efi.bin -efi-boot-part --efi-boot-image $(1) -o $(2)
endef

# 1 = limine configure features
define compile_limine_base
	cd limine && ./bootstrap
	cd limine && ./configure $(1)
	make -C limine
endef

# 1 = triple/target
# 2 = linked dependencies
# 3 = output filename
define compile_kernel
	cd kernel/ && $(CARGO) build --target triple/$(1).json --lib $(if $(KERNEL_BUILD_RELEASE), --release, )
	ld -T kernel/kernel.ld -o $(3) $(2) $(if $(KERNEL_BUILD_RELEASE), kernel/target/$(1)/release/libkernel.a, kernel/target/$(1)/debug/libkernel.a)
endef

# 1 = destination
# 2 = source
define link
$(1): $(2)
	ln -f $(2) $(1)
endef

############ x86_64 RECIEPES

RKERNEL_SRC_x86_64 = $(RKERNEL_SRC) kernel/src/arch/amd64/* kernel/triple/x86_64.json

LIMINE_ARTIFACTS_x86_64 = $(PATH_LIMINE_BIN)/limine-deploy \
													$(PATH_LIMINE_BIN)/limine.sys \
													$(PATH_LIMINE_BIN)/limine-cd-efi.bin \
													$(PATH_LIMINE_BIN)/limine-cd.bin

# these dependencies get copied on the boot partition
ISODEPS_x86_64 = build/isoroot_x86_64/kernel.bin \
								 build/isoroot_x86_64/limine-cd.bin \
								 build/isoroot_x86_64/limine-cd-efi.bin \
								 build/isoroot_x86_64/limine.sys \
								 build/isoroot_x86_64/limine.cfg 

# main
build/RezOS-x86_64.iso: $(ISODEPS_x86_64) build/limine-deploy
	$(call generate_iso_image_base, build/isoroot_x86_64/, $@)
	build/limine-deploy $@
	@echo "Done!"

$(eval $(call link, build/isoroot_x86_64/kernel.bin, 				build/kernel.x86_64.bin ))
$(eval $(call link, build/isoroot_x86_64/limine.cfg, 				kernel/limine.cfg ))
$(eval $(call link, build/isoroot_x86_64/limine-cd.bin, 		$(PATH_LIMINE_BIN)/limine-cd.bin ))
$(eval $(call link, build/isoroot_x86_64/limine-cd-efi.bin, $(PATH_LIMINE_BIN)/limine-cd-efi.bin ))
$(eval $(call link, build/isoroot_x86_64/limine.sys, 				$(PATH_LIMINE_BIN)/limine.sys ))
$(eval $(call link, build/limine-deploy, 										$(PATH_LIMINE_BIN)/limine-deploy ))

$(LIMINE_ARTIFACTS_x86_64): $(call rwildcard limine/*)
	$(call compile_limine_base, --enable-uefi-cd --enable-bios-cd)
	make -C limine limine-deploy

# the kernel itself compiles to a static library that gets linked to kentry.asm which holds the entry point and some additional structures and functions (such as limine requests)
build/kernel.x86_64.bin: build/kentry.x86_64.o $(RKERNEL_SRC_x86_64)
	$(call compile_kernel,x86_64, $<, $@)
	
build/kentry.x86_64.o: kernel/kentry/x86_64/*
	nasm -f elf64 kernel/kentry/x86_64/kentry.asm -o $@

isodeps_x86_64: $(ISODEPS_x86_64)
	echo "Built all required dependencies!"

############ COMMON RECIEPES

# visual representation of the build process
log/buildflow.png: $(PATH_MAKEFILE2GRAPH) Makefile
	make -Bnd | $(PATH_MAKEFILE2GRAPH) -r | dot -Tpng -o $@

$(PATH_MAKEFILE2GRAPH):
	cd makefile2graph && make

############ PHONY (commands, non file targets)

.PHONY: run clean deep-clean

RUN_ARGS = -D log/qemu.log -cdrom 

all: build/RezOS-x86_64.iso log/buildflow.png
	@echo "Done all jobs!"

run-x86_64: build/RezOS-x86_64.iso
	qemu-system-x86_64 $(RUN_ARGS) $^ $(QEMU_ARGS)

clean:
	find build/ -type f -delete
	rm -f log/*

deep-clean: clean
	rm -rf kernel/target/*
	rm -f $(PATH_MAKEFILE2GRAPH)
	rm -f limine/bin/*
