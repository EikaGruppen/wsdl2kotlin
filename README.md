# wsdl2kotlin

Creates wrapper klasses in Kotlin for CXF java classes generated from WSDL specifications.

## Features
- Data classes for all DTO's
  - Even for inheritance
- Enforces nullability in spec, with Kotlin nullability
- Eliminates JAXBElement bonanza

## How it works

Compiled versions for this CLI are included in `kotlin-wsdl-wrapper-maven-plugin` that can be included as standalone in different projects.

It uses [Tree-Sitter](https://tree-sitter.github.io/tree-sitter) to parse Java, then converts it, and creates Kotlin-classes.

It does some enhancements in the conversion:
- Reads `required=true` annotations on fields, and sets the nullability of the fields in the Kotlin version accordingly
- Eliminates XMLElement bonanza
- Converts inheritance to interfaces and data classes, so that every DTO have the data class abilities (automatic .equals, .copy-methods etc.)

## Use

Refer to [kotlin-wsdl-wrapper-maven-plugin]() for a maven plugin that will generate the kotlin wrapper classes

## Develop

Build
```bash
cargo build
```

Run tests
```bash
cargo test
```

### Cross-compile

This setup requires some installs, but does not require running in a docker-container, and is as a result dramatically faster.

#### Install prerequisites
1. rustup
```bash
brew uninstall rust # conflicts with rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. Install linux target
```bash
rustup target add x86_64-unknown-linux-musl
```

3. Install MUSL based CCC
```bash
brew install filosottile/musl-cross/musl-cross
ln -s /usr/local/opt/musl-cross/bin/x86_64-linux-musl-gcc /usr/local/bin/musl-gcc
```

4. LLVM

Download this, unpack, put somewhere logical, and update the path in `.cargo/config.toml`.

https://github.com/llvm/llvm-project/releases/download/llvmorg-12.0.0/clang+llvm-12.0.0-x86_64-apple-darwin.tar.xz


## TODO 

### Return enum

### Default folding of conversion code
    //<editor-fold defaultstate="collapsed" desc="conversion code between Kotlin and Java">


