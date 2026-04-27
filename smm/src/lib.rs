extern crate alloc;

pub use alloc::alloc::{GlobalAlloc, Layout};
pub use core::ptr;

pub mod mabstr;
pub mod malloc;
