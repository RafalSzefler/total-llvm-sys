total-llvm-sys
==============

This projects is a wrapper around [llvm-sys](https://crates.io/crates/llvm-sys) library.

The lib itself only reexports llvm-sys, but the build process involves downloading
and linking to the actual LLVM library.

In order to use this crate you have to specify one (and only one) of the `llvm-*`
features. For the list of currently supported `llvm-*` features
see the `Cargo.toml` file. This crate does not have default features set.
