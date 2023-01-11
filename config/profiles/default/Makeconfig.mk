# compilers the rust kernel code with the --release flag
KERNEL_BUILD_RELEASE ?= # either yes|empty

# cargo command
CARGO ?= cargo

# path to make2graph executable
PATH_MAKEFILE2GRAPH ?= makefile2graph/make2graph

# path to limine generated binaries
PATH_LIMINE_BIN ?= limine/bin/
