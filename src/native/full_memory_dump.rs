use super::*;

use std::convert::Into;

use dataview::Pod;
use log::debug;
use memflow::*;
use memflow_derive::*;

const SIZE_4KB: u64 = size::kb(4) as u64;

#[repr(C)]
#[derive(Copy, Clone, ByteSwap)]
pub struct PhysicalMemoryRun<T: Pod + ByteSwap> {
    pub base_page: T,
    pub page_count: T,
}

#[repr(C)]
#[derive(Copy, Clone, ByteSwap)]
pub struct PhysicalMemoryDescriptor<T: Pod + ByteSwap> {
    pub number_of_runs: u32,
    pub number_of_pages: T,
    pub runs: [PhysicalMemoryRun<T>; PHYSICAL_MEMORY_MAX_RUNS],
}

pub fn parse_full_dump<T: Pod + ByteSwap + Copy + Into<u64>>(
    descriptor: PhysicalMemoryDescriptor<T>,
    header_size: usize,
) -> Result<MemoryMap<(Address, usize)>> {
    let number_of_runs = descriptor.number_of_runs.into();

    if number_of_runs > PHYSICAL_MEMORY_MAX_RUNS as u64 {
        return Err(Error::Connector(
            "too many memory segments in crash dump file",
        ));
    }

    let mut mem_map = MemoryMap::new();

    // start runs from right after header size (x86: 0x1000 / x64: 0x2000)
    let mut real_base = header_size as u64;

    for i in 0..number_of_runs {
        let base = descriptor.runs[i as usize].base_page.into() * SIZE_4KB;
        let size = descriptor.runs[i as usize].page_count.into() * SIZE_4KB;

        debug!(
            "adding memory mapping: base={:x} size={:x} real_base={:x}",
            base, size, real_base
        );
        mem_map.push_remap(base.into(), size as usize, real_base.into());

        real_base += size;
    }

    Ok(mem_map)
}
