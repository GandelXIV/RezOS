############ DEFS

# https://stackoverflow.com/questions/2483182/recursive-wildcards-in-gnu-make
rwildcard=$(foreach d,$(wildcard $(1:=/*)),$(call rwildcard,$d,$2) $(filter $(subst *,%,$2),$d))

############ PATHS

# git submodules
MKBOOTIMG = bootboot/mkbootimg/mkbootimg
MAKEFILE2GRAPH = makefile2graph/make2graph

############ CONFIG

KERNEL_BUILD_PROFILE ?= '--release'
KERNEL_TRIPLE 	     ?= x86_64

############ CONDITIONAL

ifeq ($(KERNEL_BUILD_PROFILE), '--release')
	LIBKERNEL_PATH = kernel/target/$(KERNEL_TRIPLE)/release/libkernel.a 
else
	LIBKERNEL_PATH = kernel/target/$(KERNEL_TRIPLE)/debug/libkernel.a
endif

############ RECIEPE

# main
build/RezOS.bin: build/initrd.bin mkbootimg.json $(MKBOOTIMG) Makefile
	$(MKBOOTIMG) mkbootimg.json $@

build/initrd.bin: initrd/kernel.bin $(wildcard initrd/*)
	cd plexusFS/ && cargo run

initrd/kernel.bin: build/kernel.bin
	ln -f $< $@

build/kernel.bin: build/kentry.o $(wildcard kernel/* kernel/src/* kernel/src/io/* kernel/src/arch/* kernel/.cargo/* kernel/triple/*)
	cd kernel/ && cargo build --target triple/$(KERNEL_TRIPLE).json --lib $(KERNEL_BUILD_PROFILE)
	ld -T kernel/kernel.ld $< $(LIBKERNEL_PATH) -o $@
	

build/kentry.o: kernel/kentry/kentry.asm
	nasm -f elf64 $^ -o $@
log/buildflow.png: $(MAKEFILE2GRAPH) Makefile
	make -Bnd | $(MAKEFILE2GRAPH) -r | dot -Tpng -o $@


$(MKBOOTIMG):
	cd bootboot/mkbootimg && make

$(MAKEFILE2GRAPH):
	cd makefile2graph && make

############ PHONY

.PHONY: check all run clean deep-clean

check: build/kernel.bin $(MKBOOTIMG)
	# Check if kernel is bootable
	$(MKBOOTIMG) check $<

all: log/buildflow.png check build/RezOS.bin

run: build/RezOS.bin
	qemu-system-x86_64 -serial file:log/serial.log $^

clean:
	rm -f build/*
	rm -f initrd/*
	rm -f log/*

deep-clean: clean
	rm -rf kernel/target/*
	rm -rf plexusFS/target/*
	rm -f  $(MKBOOTIMG)
	rm -f  $(MAKEFILE2GRAPH)
