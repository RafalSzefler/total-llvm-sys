total-llvm-sys
==============

This projects is a wrapper around [llvm-sys](https://crates.io/crates/llvm-sys) library.

The lib itself only reexports llvm-sys, but the build process involves downloading
and linking to the actual LLVM library.

In order to use this crate you have to specify one (and only one) of the `llvm-*`
features. For the list of currently supported `llvm-*` features
see the `Cargo.toml` file. This crate does not have default features set.

Under the hood, this crate downloads LLVM binaries from a remote source, by default
releases of [llvm-builds](https://github.com/RafalSzefler/llvm-builds) repository.
It is possible that not all releases are included there. In that case you have to
build LLVM manually, zip it, upload it somewhere and set the `TOTAL_LLVM_SYS_ARCHIVE_URL_TEMPLATE`
build env variable. Follow the structure as in `.cargo/config.toml` example.
