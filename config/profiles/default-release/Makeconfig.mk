# All of these options can be change at runtime using the follow syntax:
# make [TARGETS] [OPTION1]=[VALUE1] [OPTION2]=[VALUE2] ...

# default target when none is specified
.DEFAULT_GOAL=build/RezOS-x86_64.iso

# compilers the rust kernel code with the --release flag
KERNEL_BUILD_RELEASE ?= yes # either yes|empty

# cargo command
CARGO ?= cargo

# path to make2graph executable
PATH_MAKEFILE2GRAPH ?= makefile2graph/make2graph

# path to limine generated binaries
PATH_LIMINE_BIN ?= limine/bin/
