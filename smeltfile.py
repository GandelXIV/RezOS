#!/usr/bin/python3
import sys, os
sys.path.insert(1, 'smelt3/')
import smelt3
from smelt3 import task, use, File, file_tree, shell, create_setting, sett

create_setting("KERNEL_BUILD_RELEASE", "")

### Abstract

def nasm(main: str, name: str, srcs: dict):
    for src in srcs.values():
        use(src)
    shell(f"nasm -f elf64 {main} -o {name}")
    return File(name)

def compile_rkernel(arch: str):
    use(File("kernel/Cargo.lock"))
    use(File("kernel/Cargo.toml"))
    use(File("kernel/.cargo/config.toml"))
    use(File(f"kernel/triple/{arch}.json"))
    use(file_tree("kernel/src/"))
    KERNEL_BUILD_RELEASE = sett("KERNEL_BUILD_RELEASE")

    shell(f"cd kernel/ && cargo build --target triple/{arch}.json --lib {"--release" if KERNEL_BUILD_RELEASE else ""}")
    return File(f"kernel/target/{arch}/{"release" if KERNEL_BUILD_RELEASE else "debug"}/libkernel.a")


### x86 tasks

@task("kentry")
def x86_kentry():
    return nasm(
        main = "kernel/obj/x86_64/kentry/kentry.asm",
        name = "build/kentry.o",
        srcs = file_tree("kernel/obj/x86_64/kentry/")
    )

@task()
def x86_limfeats():
    return nasm(
        main = "kernel/obj/x86_64/limfeats/limine.asm",
        name = "build/limfeats.o",
        srcs = file_tree("kernel/obj/x86_64/limfeats/")
    )

@task("kernel")
def x86_kernel():
    output = "build/kernel.x86_64.bin"
    linker_script = use(File("kernel/link/x86_64.ld"))
    kernel_obj = use(x86_kernel_object())
    kentry = use(x86_kentry())
    limfeats = use(x86_limfeats())

    shell(f"ld -T {linker_script} -o {output} {kentry} {limfeats} {kernel_obj}  ")
    return File(output)

@task()
def x86_kernel_object():
    return compile_rkernel(arch="x86_64")

### ARM tasks

@task()
def arm_kernel_object():
    return compile_rkernel(arch="aarch64")

### General tasks

@task("limine")
def limine_bootloader():
    shell("cd limine && ./bootstrap")
    shell("cd limine && ./configure --enable-bios --enable-bios-cd --enable-uefi-x86-64")
    shell("cd limine && make")
    return [File("limine/bin/limine-bios-cd.bin"), File("limine/bin/BOOTX64.EFI"), File("limine/bin/limine-bios.sys")]

@task("iso")
def isoroot():
    boots = use(limine_bootloader())
    kern = use(x86_kernel())
    limcfg = use(File("kernel/limine.conf"))

    output = "build/image.iso"
    shell("mkdir -p build/isoroot/EFI/BOOT/")

    for b in [kern, limcfg, *boots]:
        if (os.path.basename(str(b)) == "BOOTX64.EFI"):
            shell(f"cp {b} build/isoroot/EFI/BOOT/")
        else:
            shell(f"cp {b} build/isoroot/")

    shell(f"""xorriso -as mkisofs -R -r -J -b limine-bios-cd.bin \
            -no-emul-boot -boot-load-size 4 -boot-info-table -hfsplus \
            -apm-block-size 2048 --efi-boot /EFI/BOOT/BOOTX64.EFI \
            -efi-boot-part --efi-boot-image --protective-msdos-label \
            ./build/isoroot/ -o {output}
    """)

    shell(f"limine/bin/limine bios-install {output}")

    return File(output)

@task("docs")
def doc_kernel():
    use(file_tree("kernel/src/"))
    shell("cd kernel && cargo doc --document-private-items")
    return File("kernel/target/doc/kernel/index.html")

smelt3.cli()
