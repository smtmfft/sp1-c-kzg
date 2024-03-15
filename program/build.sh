#!/bin/sh

export CC=gcc
export CC_riscv32im_succinct_zkvm_elf=/opt/riscv/bin/riscv32-unknown-elf-gcc 
cargo prove build
