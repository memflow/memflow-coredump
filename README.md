# memflow-coredump

This connector implements a the Microsoft Windows Coredump format for 32-bit and 64-bit Coredump files. It implements support for full core dumps (type 1) and partial bit mapped core dumps (type 5).

## Compilation

### Using the crate in a rust project

To use the connector in a rust project just include it in your Cargo.toml

```
memflow-coredump = "0.1"
```

Make sure to not enable the `inventory` feature when importing multiple
connectors in a rust project without using the memflow connector inventory.
This might cause duplicated exports being generated in your project.

### Building the stand-alone connector (for dynamic loading)

The stand-alone connector of this library is feature-gated behind the `inventory` feature.
To compile a dynamic library for use with the connector inventory use the following command:

```cargo build --release --all-features```

## Arguments

- `file` - the path of the coredump file to open (default argument)

## License

Licensed under MIT License, see [LICENSE](LICENSE).

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, shall be licensed as above, without any additional terms or conditions.
