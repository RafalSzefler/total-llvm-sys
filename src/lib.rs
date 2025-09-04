//! This crate simply reexports the llvm-sys crate. But under the hood
//! it also automatically downloads and links to the actually LLVM library.
#![deny(warnings)]
#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::inline_always)]

mod macros;

macros::valid_targets!("x86_64", "x86", "aarch64", "arm");

macros::reexport_llvm!();

/// This module contains additional info about LLVM version,
/// current architecture and operating system.
pub mod meta;

/// This module contains helper functions for native (current) target.
pub mod native;
