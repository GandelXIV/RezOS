# RezOS
A modern operating system written in the rust programming language.
### premise:
- Rust as a primary language for safety, speed and security
- Modern bootloader support with the limine protocol
- Architectural compatibility for x86_64 and AARCH64/ARM64
- Innovative approach to Micro/Monolithic kernel design
- Graph VFS
- Capability based permissions
- Proper async IO interface
- Running DOOM
- Full network & graphics stack

# Setup
Note: This guide assumes you do this on linux. <br>
Follow these steps: <br>
1. Clone the source repository <br>
`git clone https://www.github.com/GandelXIV/RezOS.git`
2. Update git submodules <br>
`git submodule update --init`
3. Install build dependencies <br>
  There are two ways to do this:
  - Using an install script located in `scripts/install/all/{your-distro}.sh`. <br> Note that currently only Arch and Mint(Ubuntu) are supported. <br>
    If you have a different distro you can still use `scripts/install/rust-linux.sh` to setup rust, then continue with the following step to install the rest.
  - Install manually as described in `scripts/install/packages/packages.md` and `scripts/install/rust-linux.sh`.
4. Configure either manually(Docs to be added) or via: <br>
`./configure.sh`
5. Build with make <br>
`make`
6. Run/debug in emulator <br>
`make run`

