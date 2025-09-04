total-llvm-sys
==============

This projects is a wrapper around [llvm-sys](https://crates.io/crates/llvm-sys) library.

The lib itself only reexports llvm-sys, but the build process involves downloading
and linking to the actual LLVM library.
