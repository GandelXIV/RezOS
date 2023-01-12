# Configuration
Before building the OS, you need to create a few configuration files.
This can be done manually or by running the `config/config.sh` script from the project root.
The way it works is that it links files from `profiles/` to their correct location as described in `list.json`. The script requires you to pass it as a command line argument the name of the profile. It also creates build directories.

## How to change configuration
To change the configuration you need to first deconfigure with the `config/deconfig.sh` script. Note that it must be run from the project root! After that, you can reconfig with any profile you want.

## Creating profiles

If you wish to create your own profile, simply copy the `default` directory, rename it and change whatever you wish. 

## Default config files explained
A deeper description can usually be seen in the file itself.

- `Makeconfig.mk` : Build system options. Allows setting compile optimizations & paths.
- `rkernel.rs` : Holds most variable settings (number/text)
- `kentry-aarch64.S` : lower-level settings for the kernel. Only for aarch64/arm64.
- `kentry-x86_64.asm` : lower-level settings for the kernel. Only for x86_64/amd64.

