# memflow-coredump

The `coredump` connector implements the Microsoft Windows Coredump format for 32-bit and 64-bit Coredump files. It implements support for full core dumps (type 1) and partial bit mapped core dumps (type 5).

## Compilation

### Installing the library

The recommended way to install memflow connectors is using [memflowup](https://github.com/memflow/memflowup#memflow-setup-tool).

### Development builds

To compile the connector as dynamic library to be used with the memflow plugin system use the following command:

```
cargo build --release --all-features
```

The plugin can then be found in the `target/release/` directory and has to be copied to one of [memflows default search paths](https://github.com/memflow/memflow/blob/main/memflow/src/plugins/mod.rs#L379).

### Linking the crate statically in a rust project

To use the connector in a rust project just include it in your Cargo.toml

```
memflow-coredump = "^0.2.0-beta"
```

## Arguments

The `target` argument specifies the filename of the coredump file to be opened.

## License

Licensed under MIT License, see [LICENSE](LICENSE).

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, shall be licensed as above, without any additional terms or conditions.
