############ DEFS

# https://stackoverflow.com/questions/2483182/recursive-wildcards-in-gnu-make
rwildcard=$(foreach d,$(wildcard $(1:=/*)),$(call rwildcard,$d,$2) $(filter $(subst *,%,$2),$d))

############ PATHS

MAKEFILE2GRAPH = makefile2graph/make2graph
LIMINE_BIN = limine/bin/
RKERNEL_SRC = $(wildcard kernel/* kernel/src/* kernel/src/memman/* kernel/src/arch/* kernel/src/arch/amd64/* kernel/.cargo/* kernel/triple/*) 

############ OPTIONS

# either x86_64 | aarch64
TARGET_ARCH ?= x86_64

# either on/off
KERNEL_BUILD_WITH_RELEASE ?= off 

# can be set to another path
CARGO ?= cargo

############ CONDITIONS

KERNEL_TRIPLE = $(TARGET_ARCH)

ifeq ($(KERNEL_BUILD_WITH_RELEASE), on) 
	RKERNEL_PATH = kernel/target/$(KERNEL_TRIPLE)/release/libkernel.a 
	KERNEL_BUILD_RELEASE = '--release'
else
	RKERNEL_PATH = kernel/target/$(KERNEL_TRIPLE)/debug/libkernel.a
	KERNEL_BUILD_RELEASE =
endif


ifeq ($(TARGET_ARCH), x86_64)
	LIMINE_CONFIGURE_FEATURES = --enable-uefi-cd --enable-bios-cd
endif

ifeq ($(TARGET_ARCH), aarch64)
	LIMINE_CONFIGURE_FEATURES = --enable-uefi-aarch64
endif


############ RECIEPE

# these dependencies get copied on the boot partition
ISODEPS = isoroot/kernel.bin isoroot/limine-cd.bin isoroot/limine-cd-efi.bin isoroot/limine.sys isoroot/limine.cfg build/limine-deploy  

# main
build/RezOS.iso: scripts/mk/mkiso.sh $(ISODEPS)
	$< $@
	@echo "Done!"

isodeps: $(ISODEPS)
	echo "Built all required dependencies!"

isoroot/kernel.bin: build/kernel.bin
	ln -f $< $@

isoroot/limine-cd.bin: $(LIMINE_BIN)/limine-cd.bin
	ln -f $< $@

isoroot/limine-cd-efi.bin: $(LIMINE_BIN)/limine-cd-efi.bin 
	ln -f $< $@

isoroot/limine.sys: $(LIMINE_BIN)/limine.sys 
	ln -f $< $@

isoroot/limine.cfg: kernel/limine.cfg 
	ln -f $< $@

build/limine-deploy: $(LIMINE_BIN)/limine-deploy 
	ln -f $< $@

$(LIMINE_BIN)/limine-deploy $(LIMINE_BIN)/limine.sys $(LIMINE_BIN)/limine-cd-efi.bin $(LIMINE_BIN)/limine-cd.bin: $(call rwildcard limine/*)
	cd limine && ./bootstrap
	cd limine && ./configure $(LIMINE_CONFIGURE_FEATURES)
	make -C limine
	make -C limine limine-deploy

# the kernel itself compiles to a static library that gets linked to kentry.asm which holds the entry point and some additional structures and functions (such as limine requests)
build/kernel.bin: build/kentry.o $(RKERNEL_SRC)	
	cd kernel/ && $(CARGO) build --target triple/$(KERNEL_TRIPLE).json --lib $(KERNEL_BUILD_RELEASE)
	ld -T kernel/kernel.ld $< $(RKERNEL_PATH) -o $@
	
build/kentry.o: kernel/kentry/kentry.asm kernel/kentry/limine.asm
	nasm -f elf64 $< -o $@

# visual representation of the build process
log/buildflow.png: $(MAKEFILE2GRAPH) Makefile
	make -Bnd | $(MAKEFILE2GRAPH) -r | dot -Tpng -o $@

$(MAKEFILE2GRAPH):
	cd makefile2graph && make

############ PHONY (commands, non file targets)

.PHONY: run clean deep-clean

RUN_ARGS = -D log/qemu.log -cdrom 

run: build/RezOS.iso
	qemu-system-x86_64 $(RUN_ARGS) $^ $(QEMU_ARGS)

run-spice: build/RezOS.iso
	qemu-system-x86_64 $(RUN_ARGS) $^ -display spice-app $(QEMU_ARGS)

clean:
	rm -f build/*
	rm -f isoroot/*
	rm -f log/*

deep-clean: clean
	rm -rf kernel/target/*
	rm -f  $(MAKEFILE2GRAPH)
	rm -f limine/bin/*
