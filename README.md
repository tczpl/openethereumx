# OpenEthereumX

## Make OpenEthereum Great Again!

OpenEthereum's goal is to be the fastest, lightest, and most secure Ethereum client. OpenEthereum is licensed under the GPLv3 and can be used for all your Ethereum needs.

Unfortunately, OpenEthereum is deprecated. However, many workflows depend on OpenEthereum's interfaces.

OpenEthereumX implements several hard forks (***The Merge, Shanghai-Capella, Dencun***) of the execution layer so that nodes can still keep up with the mainnet blocks (with the ```import``` command).

## Building OpenEthereumX

### Build Dependencies

OpenEthereumX requires **latest stable Rust version** to build.

We recommend installing Rust through [rustup](https://www.rustup.rs/). If you don't already have `rustup`, you can install it like this:

- Linux:
  ```bash
  $ curl https://sh.rustup.rs -sSf | sh
  $ rustup override set 1.64.0
  ```

  OpenEthereumX also requires `clang` (>= 9.0), `clang++`, `pkg-config`, `file`, `make`, and `cmake` packages to be installed.

Make sure that these binaries are in your `PATH`. After that, you should be able to build OpenEthereumX from source.

### Build from Source Code

```bash
# download OpenEthereumX code
$ git clone https://github.com/tczpl/openethereumx
$ cd openethereumx

# build in release mode
$ cargo build --release --features final
```

This produces an executable in the `./target/release` subdirectory.

Note: if cargo fails to parse manifest try:

```bash
$ ~/.cargo/bin/cargo build --release
```

Note, when compiling a crate and you receive errors, it's in most cases your outdated version of Rust, or some of your crates have to be recompiled. Cleaning the repository will most likely solve the issue if you are on the latest stable version of Rust, try:

```bash
$ cargo clean
```
