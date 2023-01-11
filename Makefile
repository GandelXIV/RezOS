# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at https://mozilla.org/MPL/2.0/.

############ OPTIONS

include config.mk

############ GENERICS

RKERNEL_SRC = kernel/Cargo* \
							kernel/rust-toolchain \
							kernel/src/* \
							kernel/src/memman/* \
							kernel/src/arch/* \
							kernel/.cargo/* \

# https://stackoverflow.com/questions/2483182/recursive-wildcards-in-gnu-make
rwildcard=$(foreach d,$(wildcard $(1:=/*)),$(call rwildcard,$d,$2) $(filter $(subst *,%,$2),$d))

# 1 = limine configure features
define compile_limine_base
	cd limine && ./bootstrap
	cd limine && make distclean || true
	cd limine && ./configure $(1)
	make -C limine
endef

# 1 = triple/target
# 2 = linked dependencies
# 3 = output filename
# 4 = linker
define compile_kernel
	cd kernel/ && $(CARGO) build --target triple/$(1).json --lib $(if $(KERNEL_BUILD_RELEASE), --release, )
	$(4) -T kernel/link/$(1).ld -o $(3) $(2) $(if $(KERNEL_BUILD_RELEASE), kernel/target/$(1)/release/libkernel.a, kernel/target/$(1)/debug/libkernel.a)
endef

# 1 = triple/target
define document_kernel
	cd kernel/ && $(CARGO) doc --document-private-items --target triple/$(1).json
endef

# 1 = destination
# 2 = source
define link
$(1): $(2)
	ln -f $(2) $(1)
endef

# 1 = destination
# 2 = source
define symlink
$(1): $(2)
	rm -f  $(1)
	ln -sf $(2) $(1)
endef


############ x86_64 RECIEPES

RKERNEL_SRC_x86_64 = $(RKERNEL_SRC) kernel/src/arch/amd64/* kernel/triple/x86_64.json kernel/link/x86_64.ld

RKERNEL_DOC_x86_64 = kernel/target/x86_64/doc/kernel/

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
	xorriso -as mkisofs -b limine-cd.bin \
					-no-emul-boot \
					-boot-load-size 4 \
					-boot-info-table --efi-boot limine-cd-efi.bin -efi-boot-part \
					--efi-boot-image \
					build/isoroot_x86_64/ -o $@
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
	$(call compile_kernel,x86_64, $<, $@, ld)
	
build/kentry.x86_64.o: kernel/kentry/x86_64/*
	nasm -f elf64 kernel/kentry/x86_64/kentry.asm -o $@

isodeps_x86_64: $(ISODEPS_x86_64)
	echo "Built all required dependencies!"

# docs

$(RKERNEL_DOC_x86_64): $(RKERNEL_SRC_x86_64)
	$(call document_kernel,x86_64)

############ aarch64 RECIEPES

RKERNEL_SRC_aarch64 = $(RKERNEL_SRC) kernel/src/arch/arm64/* kernel/triple/aarch64.json kernel/link/aarch64.ld

RKERNEL_DOC_aarch64 = kernel/target/aarch64/doc/kernel/

ISODEPS_aarch64 = build/isoroot_aarch64/kernel.bin \
									build/isoroot_aarch64/limine.cfg \
									build/isoroot_aarch64/BOOTAA64.EFI

build/RezOS-aarch64.iso: $(ISODEPS_aarch64)
	xorriso -as mkisofs \
					-no-emul-boot \
					-boot-info-table \
					--efi-boot BOOTAA64.EFI -efi-boot-part \
					--efi-boot-image \
					build/isoroot_aarch64/ -o $@
	@echo "Done!"

$(eval $(call link, build/isoroot_aarch64/kernel.bin, 	build/kernel.aarch64.bin ))
$(eval $(call link, build/isoroot_aarch64/limine.cfg, 	kernel/limine.cfg ))
$(eval $(call link, build/isoroot_aarch64/BOOTAA64.EFI, $(PATH_LIMINE_BIN)/BOOTAA64.EFI ))

$(PATH_LIMINE_BIN)/BOOTAA64.EFI: $(call rwildcard limine/*)
	$(call compile_limine_base, --enable-uefi-aarch64)

build/kernel.aarch64.bin: build/kentry.aarch64.o $(RKERNEL_SRC_aarch64)
	$(call compile_kernel,aarch64, $<, $@, aarch64-linux-gnu-ld)

build/kentry.aarch64.o: kernel/kentry/aarch64/*
	aarch64-linux-gnu-as kernel/kentry/aarch64/kentry.S -o $@

# docs

$(RKERNEL_DOC_aarch64): $(RKERNEL_SRC_aarch64)
	$(call document_kernel,aarch64)

############ COMMON RECIEPES

# visual representation of the build process
doc/buildflow.png: $(PATH_MAKEFILE2GRAPH) Makefile
	make all -Bnd | $(PATH_MAKEFILE2GRAPH) -r | dot -Tpng -o $@

$(PATH_MAKEFILE2GRAPH):
	cd makefile2graph && make

############ PHONY (commands, non file targets)

.PHONY: run clean deep-clean all doc

RUN_ARGS = -D log/qemu.log -cdrom 

all: build/RezOS-x86_64.iso build/RezOS-aarch64.iso doc
	@echo "Done all jobs!"

doc: doc/buildflow.png $(RKERNEL_DOC_aarch64) $(RKERNEL_DOC_x86_64)
	@echo "Documentation generated!"

run-x86_64: build/RezOS-x86_64.iso
	qemu-system-x86_64 $(RUN_ARGS) $^ $(QEMU_ARGS)

clean:
	find build/ -type f -delete
	rm -f log/*
	rm -f doc/buildflow.png

deep-clean: clean clean-limine
	rm -rf kernel/target/
	rm -f $(PATH_MAKEFILE2GRAPH)

clean-limine:
	cd limine
	rm -f limine/bin/*
	rm -f limine/cross-files/config.log
	rm -f limine/cross-files/config.status
	rm -f limine/cross-files/i686-toolchain.mk

distclean-limine:
	cd limine && make distclean
