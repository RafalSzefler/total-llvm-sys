//! This crate simply reexports the llvm-sys crate. But under the hood
//! it also automatically downloads and links to the actually LLVM library.
#![deny(warnings)]
#![warn(clippy::all, clippy::pedantic)]
#![allow(
    clippy::inline_always,
)]

mod macros;

macros::valid_targets!("x86_64", "x86", "aarch64", "arm", "loongarch64");

#[cfg(feature = "llvm-19")]
pub extern crate llvm_sys_19 as llvm_sys;

/// This module contains helper functions for native (current) target.
pub mod native;
