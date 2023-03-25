mod native;
use native::*;

use std::fs::{File, OpenOptions};
use std::path::Path;

use memflow::prelude::v1::*;

/**
The `parse_file` function reads and parses Microsoft Windows Coredump files.

When opening a crashdump it tries to parse the first 0x2000 bytes of the file as a 64 bit Windows Coredump.
If the validation of the 64 bit Header fails it tries to read the first 0x1000 bytes of the file and validates it as 32 bit Windows Coredump.

If neither attempt succeeds the function will fail with an `Error::Conector` error.

`create_connector` function attempts to directly create a connector (based on crate configuration - mmap or stdio based).

# Examples

```
use std::path::PathBuf;

use memflow::plugins::ConnectorArgs;
use memflow_coredump::create_connector;

let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    .join("resources/test/coredump_win10_64bit_stripped.raw");
if let Ok(mut mem) = create_connector(&str::parse(path.to_str().unwrap()).expect("unable to parse command line arguments")) {
    println!("Coredump connector initialized");
}
```
*/

#[cfg(feature = "filemap")]
pub type CoreDump<'a> = ReadMappedFilePhysicalMemory<'a>;
#[cfg(not(feature = "filemap"))]
pub type CoreDump<'a> = FileIOMemory<File>;

/// Opens a Microsoft Windows Coredump
///
/// This function will return the underlying file and the memory map with correct file offsets.
/// These arguments can then be passed to the mmap or read connector for Read/Write operations.
pub fn parse_file<P: AsRef<Path>>(path: P) -> Result<(MemoryMap<(Address, umem)>, File)> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(false)
        .open(path)
        .map_err(|_| {
            Error(ErrorOrigin::Connector, ErrorKind::Unknown)
                .log_error("unable to open coredump file")
        })?;

    let mem_map = parse_coredump64(&mut file).or_else(|_| parse_coredump32(&mut file))?;

    Ok((mem_map, file))
}

/// Creates a new Microsoft Windows Coredump Connector instance.
///
/// This function will return a connector reading the underlying data of the core dump.
/// The type of connector depends on the feature flags of the crate.
#[connector(name = "coredump", help_fn = "help", no_default_cache = true)]
pub fn create_connector<'a>(args: &ConnectorArgs) -> Result<CoreDump<'a>> {
    let (map, file) = parse_file(args.target.as_deref().ok_or_else(|| {
        Error(ErrorOrigin::Connector, ErrorKind::Unknown).log_error("`file` argument missing")
    })?)?;
    #[cfg(feature = "filemap")]
    {
        Ok(MmapInfo::try_with_filemap(file, map)?.into_connector())
    }
    #[cfg(not(feature = "filemap"))]
    {
        Ok(CoreDump::try_with_reader(file, map)?.into_connector())
    }
}

/// Retrieve the help text for the Qemu Procfs Connector.
pub fn help() -> String {
    "\
The `coredump` connector implements the Microsoft Windows Coredump format
for 32-bit and 64-bit Coredump files. It implements support for
full core dumps (type 1) and partial bit mapped core dumps (type 5).

The `target` argument specifies the filename of the coredump file to be opened."
        .to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn parse_win10_64bit() {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("resources/test/coredump_win10_64bit_stripped.raw");
        parse_file(path).unwrap();
    }

    #[test]
    fn parse_win7_32bit() {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("resources/test/coredump_win7_32bit_stripped.raw");
        parse_file(path).unwrap();
    }
}
