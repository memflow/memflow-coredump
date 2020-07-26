# memflow-coredump

This connector implements a the Microsoft Windows Coredump format for 32-bit and 64-bit Coredump files. It implements support for full core dumps (type 1) and partial bit mapped core dumps (type 5).

## Compilation

### Using the library in a rust project

To use the plugin in a rust project just include it in your Cargo.toml

```
memflow-coredump = "0.1"
```

Make sure to not enable the `plugin` feature when importing multiple
connectors in a rust project without using the memflow plugin inventory.
This might cause duplicated exports being generated in your project.

### Building the stand-alone plugin

The stand-alone plugin of this library is feature-gated behind the `plugin` feature.
To compile a dynamic library as a plugin use the following command:

```cargo build --release --all-features```

## License

Licensed under MIT License, see [LICENSE](LICENSE).

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, shall be licensed as above, without any additional terms or conditions.
