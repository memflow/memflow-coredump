pub mod x64;
pub mod x86;

pub mod bitmap_dump;
pub mod full_memory_dump;

use std::{
    io::Read,
    mem::{size_of, MaybeUninit},
    slice,
};

pub use x64::parse_coredump64;
pub use x86::parse_coredump32;

use memflow::prelude::v1::*;

/// Coredump Header Signature
pub const DUMP_SIGNATURE: u32 = 0x45474150;

/// The type of the Coredump
mod dump_type {
    pub const FULL: u32 = 1;
    pub const BIT_MAP: u32 = 5;
}

/// The number of PhysicalMemoryRuns contained in the Header
pub const PHYSICAL_MEMORY_MAX_RUNS: usize = 0x20;

pub(crate) unsafe fn file_read_raw_struct<R: Read, T: Sized>(src: &mut R) -> Result<T> {
    let mut buffer = MaybeUninit::uninit();
    let buffer_slice = slice::from_raw_parts_mut(buffer.as_mut_ptr() as *mut u8, size_of::<T>());

    src.read_exact(buffer_slice).map_err(|_| {
        Error(ErrorOrigin::Connector, ErrorKind::Unknown).log_error("unable to read bitmap header")
    })?;

    Ok(buffer.assume_init())
}
