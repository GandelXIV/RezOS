############ DEFS

# https://stackoverflow.com/questions/2483182/recursive-wildcards-in-gnu-make
rwildcard=$(foreach d,$(wildcard $(1:=/*)),$(call rwildcard,$d,$2) $(filter $(subst *,%,$2),$d))

############ PATHS

MAKEFILE2GRAPH = makefile2graph/make2graph
LIMINE_BIN = limine/bin/

############ CONFIG

KERNEL_BUILD_PROFILE ?= '--release'
KERNEL_TRIPLE 	     ?= x86_64

############ CONDITIONAL

ifeq ($(KERNEL_BUILD_PROFILE), '--release')
	RKERNEL_PATH = kernel/target/$(KERNEL_TRIPLE)/release/libkernel.a 
else
	RKERNEL_PATH = kernel/target/$(KERNEL_TRIPLE)/debug/libkernel.a
endif

############ RECIEPE

ISODEPS = isoroot/kernel.bin isoroot/limine-cd.bin isoroot/limine-cd-efi.bin isoroot/limine.sys isoroot/limine.cfg build/limine-deploy  

# main
build/RezOS.iso: scripts/mk/mkiso.sh $(ISODEPS)
	$< $@

isodeps: $(ISODEPS)
	echo "Build all required dependencies!"

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
	cd limine && ./configure --enable-bios-cd --enable-uefi-cd
	make -C limine
	make -C limine limine-deploy

build/kernel.bin: build/kentry.o $(wildcard kernel/* kernel/src/* kernel/src/io/* kernel/src/arch/* kernel/.cargo/* kernel/triple/*)
	cd kernel/ && cargo build --target triple/$(KERNEL_TRIPLE).json --lib $(KERNEL_BUILD_PROFILE)
	ld -T kernel/kernel.ld $< $(RKERNEL_PATH) -o $@
	
build/kentry.o: kernel/kentry/kentry.asm
	nasm -f elf64 $^ -o $@

log/buildflow.png: $(MAKEFILE2GRAPH) Makefile
	make -Bnd | $(MAKEFILE2GRAPH) -r | dot -Tpng -o $@

$(MAKEFILE2GRAPH):
	cd makefile2graph && make

############ PHONY

.PHONY: run clean deep-clean

run: build/RezOS.iso
	qemu-system-x86_64 -D log/qemu.log -cdrom $^ $(QEMU_ARGS)

clean:
	rm -f build/*
	rm -f isoroot/*
	rm -f log/*

deep-clean: clean
	rm -rf kernel/target/*
	rm -f  $(MAKEFILE2GRAPH)
	rm -f limine/bin/*
