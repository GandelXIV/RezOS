.PHONY: clean all run check

# https://stackoverflow.com/questions/2483182/recursive-wildcards-in-gnu-make
rwildcard=$(foreach d,$(wildcard $(1:=/*)),$(call rwildcard,$d,$2) $(filter $(subst *,%,$2),$d))

MKBOOTIMG = bootboot/mkbootimg/mkbootimg

build/RezOS.bin: build/initrd.bin mkbootimg.json $(MKBOOTIMG)
	$(MKBOOTIMG) mkbootimg.json $@

build/initrd.bin: initrd/kernel.bin $(wildcard initrd/*)
	cd plexusFS/ && cargo run

initrd/kernel.bin: build/kernel.bin
	ln -f $< $@

build/kernel.bin: build/kentry.o $(wildcard kernel/* kernel/src/* kernel/src/io/* kernel/.cargo/* kernel/triple/*)
	cd kernel/ && cargo build --target triple/x86_64.json --lib --release
	ld -T kernel/kernel.ld $< kernel/target/x86_64/release/libkernel.a -o $@
	
build/kentry.o: kernel/kentry/kentry.asm
	nasm -f elf64 $^ -o $@

$(MKBOOTIMG):
	cd bootboot/mkbootimg && make

check: build/kernel.bin
	$(MKBOOTIMG) check $<

all: check build/RezOS.bin

run: build/RezOS.bin
	qemu-system-x86_64 -serial file:log/serial.log $^

clean:
	rm -f build/*
	rm -f initrd/kernel.bin