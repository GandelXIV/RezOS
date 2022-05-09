.PHONY: clean run

ASM = nasm
ASM_FORMAT = bin
CARGO = cargo
EMU = qemu-system-x86_64

build/RezOS.bin: build/boot.bin build/mkfs.exec
	build/mkfs.exec -b build/boot.bin -o $@

build/mkfs.exec: mkfs/* Cargo.toml
	$(CARGO) build --bin mkfs --release
	cp target/release/mkfs $@

build/boot.bin: boot/*
	$(ASM) -f $(ASM_FORMAT) boot/main.asm -o $@

clean:
	rm -f build/*

run: build/RezOS.bin
	$(EMU) build/RezOS.bin
