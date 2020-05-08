//! # btrfsutil-sys
//!
//! [![Build Status](https://travis-ci.com/cezarmathe/btrfsutil-sys.svg?branch=master)](https://travis-ci.com/cezarmathe/btrfsutil-sys)
//! [![btrfsutil-sys](https://img.shields.io/crates/v/btrfsutil-sys)](https://crates.io/crates/btrfsutil-sys)
//! [![docs](https://docs.rs/btrfsutil-sys/badge.svg)](https://docs.rs/btrfsutil-sys)
//! [![libbtrfsutil version](https://img.shields.io/badge/libbtrfsutil-1.2.0-7979F1)](https://github.com/kdave/btrfs-progs/blob/471b4cf7e3a46222531a895f90228ea164b1b857/libbtrfsutil/btrfsutil.h#L28-L30)
//!
//! Raw bindings to [libbtrfsutil](https://github.com/kdave/btrfs-progs/tree/master/libbtrfsutil).
//!
//! ## Building
//!
//! This library links to `libbtrfsutil`, a shared library provided by installing [btrfs-progs](https://github.com/kdave/btrfs-progs) on most Linux systems.
//!
//! - Arch Linux: `# pacman -S btrfs-progs`
//! - Ubuntu: `# apt install libbtrfsutil1` (available since [eoan](https://releases.ubuntu.com/19.10/))
//!
//! ## Usage
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! btrfsutil-sys = "1.2.1"
//! ```
//!
//! For further details, please refer to the [documentation](https://docs.rs/btrfsutil-sys).
//!
//! Also, please keep in mind that many of the operations this library can perform may require elevated
//! privileges(CAP_SYSTEM_ADMIN).
//!
//! ## License
//!
//! MIT

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

/// Id of the root subvolume in a Btrfs filesystem.
pub const BTRFS_FS_TREE_OBJECTID: u64 = 5;
