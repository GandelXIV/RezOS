.PHONY: clean run

ASM = nasm
ASM_FORMAT = bin
CARGO = cargo
EMU = qemu-system-x86_64
EMU_ARGS = 

build/RezOS.bin: build/boot.bin build/mkfs.exe build/kernel.bin
	build/mkfs.exe

build/mkfs.exe: mkfs/* Cargo.toml
	$(CARGO) build --bin mkfs --release
	cp target/release/mkfs $@

build/boot.bin: boot/* boot/real/io/* boot/real/* 
	$(ASM) -f $(ASM_FORMAT) boot/main.asm -o $@

clean:
	rm -f build/*

run: build/RezOS.bin
	killall $(EMU) &	# need to kill past instances of EMU so we dont connect to them accidentally
	$(EMU) $(EMU_ARGS) build/RezOS.bin &
	sleep 0.1
	vncviewer vncviewer 127.0.0.1:5900

build/kernel.bin:
	echo "no kernel for you trololol" > $@