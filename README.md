# OpenEthereumX

## Make OpenEthereum Great Again!

## 1. Description <a id="chapter-001"></a>

**Built for mission-critical use**: Miners, service providers, and exchanges need fast synchronisation and maximum uptime. OpenEthereum provides the core infrastructure essential for speedy and reliable services.

- Clean, modular codebase for easy customisation
- Advanced CLI-based client
- Minimal memory and storage footprint
- Synchronise in hours, not days with Warp Sync
- Modular for light integration into your service or product

## 2. Technical Overview <a id="chapter-002"></a>

OpenEthereum's goal is to be the fastest, lightest, and most secure Ethereum client. We are developing OpenEthereum using the **Rust programming language**. OpenEthereum is licensed under the GPLv3 and can be used for all your Ethereum needs.

By default, OpenEthereum runs a JSON-RPC HTTP server on port `:8545` and a Web-Sockets server on port `:8546`. This is fully configurable and supports a number of APIs.

If you run into problems while using OpenEthereum, check out the [old wiki for documentation](https://openethereum.github.io/), feel free to [file an issue in this repository](https://github.com/openethereum/openethereum/issues/new), or hop on our [Discord](https://discord.io/openethereum) chat room to ask a question. We are glad to help!

You can download OpenEthereum's latest release at [the releases page](https://github.com/openethereum/openethereum/releases) or follow the instructions below to build from source. Read the [CHANGELOG.md](CHANGELOG.md) for a list of all changes between different versions.

## 3. Building <a id="chapter-003"></a>

### 3.1 Build Dependencies <a id="chapter-0031"></a>

OpenEthereum requires **latest stable Rust version** to build.

We recommend installing Rust through [rustup](https://www.rustup.rs/). If you don't already have `rustup`, you can install it like this:

- Linux:
  ```bash
  $ curl https://sh.rustup.rs -sSf | sh
  $ rustup override set 1.64.0
  ```

  OpenEthereum also requires `clang` (>= 9.0), `clang++`, `pkg-config`, `file`, `make`, and `cmake` packages to be installed.

Make sure that these binaries are in your `PATH`. After that, you should be able to build OpenEthereum from source.

### 3.2 Build from Source Code <a id="chapter-0032"></a>

```bash
# download OpenEthereum code
$ git clone https://github.com/openethereum/openethereum
$ cd openethereum

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
