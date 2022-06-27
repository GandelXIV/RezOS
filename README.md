# RezOS
A modern operating system written in the rust programming language.
# Installation
0. Prerequisite: Linux with bash
1. Clone the repo
`git clone https://www.github.com/GandelXIV/RezOS.git`
2. Update git submodules
`git submodule update --init`
3. Install the following dependencies:
`make nasm rust cargo qemu gcc`
4. Configure either manually or with:
`./configure.sh`
5. Build
`make`
6. Run with emulator
`make run`
