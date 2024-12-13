#!/usr/bin/python3
import sys, os
sys.path.insert(1, 'smelt3/')
import smelt3
from smelt3 import task, use, File, file_tree, shell, create_setting, sett

create_setting("KERNEL_BUILD_RELEASE", False)

@task("kernel")
def x86_kernel():
    output = "build/kernel.x86_64.bin"
    linker_script = use(File("kernel/link/x86_64.ld"))
    kernel_obj = use(x86_kernel_object())
    kentry = use(x86_kentry())

    shell(f"ld -T {linker_script} -o {output} {kentry} {kernel_obj}  ")
    return File(output)

@task()
def x86_kentry():
    out = "build/kentry.x86_64.o"
    use(file_tree("kernel/kentry/x86_64/"))

    shell("mkdir -p build/")
    shell(f"nasm -f elf64 kernel/kentry/x86_64/kentry.asm -o {out}")
    return File(out)

@task()
def x86_kernel_object():
    use(File("kernel/Cargo.lock"))
    use(File("kernel/Cargo.toml"))
    use(File("kernel/link/x86_64.ld"))
    use(File("kernel/.cargo/config.toml"))
    use(File("kernel/triple/x86_64.json"))
    use(file_tree("kernel/src/"))
    KERNEL_BUILD_RELEASE = sett("KERNEL_BUILD_RELEASE")

    shell(f"cd kernel/ && cargo build --target triple/x86_64.json --lib {"--release" if KERNEL_BUILD_RELEASE else ""}")
    return File(f"kernel/target/x86_64/{"release" if KERNEL_BUILD_RELEASE else "debug"}/libkernel.a")

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

smelt3.cli()
