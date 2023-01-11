# Configuration
Before building the OS, you need to create a few configuration files.
This can be done manually or using the `./config.sh` script.
The way it works is that it links files from `profiles/` to their correct location as described in `list.json`. The script requires you to pass it as a command line argument the name of the profile.

## How to change configuration
To change the configuration you need to first deconfigure with the `./deconfig.sh` script. After that, you can reconfig with any profile you want.

## Creating profiles

If you wish to create your own profile, simply copy the `default` directory, rename it and change whatever you wish. 
