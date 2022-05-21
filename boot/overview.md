# Boot overview
## stage 1 = Load stage 2 (currently not needed)
## stage 2 = Basic prep
- load segment registers
- setup stack
- enter rmain
## stage 3 = Load kernel
- read fs & use directboot
- load sectors from disk
## stage 4 = Gather info
- get mmap sizes
- disks
- cpu
- avaible video modes
## stage 5 = kernel env
- create temporary gdt
- switch to long/protected mode
## stage 6 = final
- Give control to the kernel