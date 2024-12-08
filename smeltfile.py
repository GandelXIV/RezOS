#!/usr/bin/python3
import sys
sys.path.insert(1, 'smelt3/')
import smelt3
from smelt3 import task, use, File, file_tree, shell, create_setting, sett

create_setting("KERNEL_BUILD_RELEASE", False)

@task("kentry")
def x86_kentry():
    out = "build/kentry.x86_64.o"
    use(file_tree("kernel/kentry/x86_64/"))

    shell("mkdir -p build/")
    shell(f"nasm -f elf64 kernel/kentry/x86_64/kentry.asm -o {out}")
    return File(out)

@task("kernel_object")
def x86_kernel_object():
    use(File("kernel/Cargo.lock"))
    use(File("kernel/Cargo.toml"))
    use(File("kernel/link/x86_64.ld"))
    use(File("kernel/.cargo/config.toml"))
    use(File("kernel/triple/x86_64.json"))
    use(file_tree("kernel/src/"))
    KERNEL_BUILD_RELEASE = sett("KERNEL_BUILD_RELEASE")

    shell(f"cd kernel/ && cargo build --target triple/x86_64.json --lib {"--release" if KERNEL_BUILD_RELEASE else ""}")

    return File(f"kernel/target/{"release" if KERNEL_BUILD_RELEASE else "debug"}/release/libkernel.a")

smelt3.cli()
