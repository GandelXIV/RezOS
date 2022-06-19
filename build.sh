cd kernel/
cargo build --target triple/x86_64.json
nasm -f elf64 entry.asm -o ../build/kentry.o 
ld -T kernel.ld ../build/kentry.o target/x86_64/debug/libkernel.rlib -o ../build/kernel.bin

~/repos/bootboot/mkbootimg/mkbootimg check ../build/kernel.bin
