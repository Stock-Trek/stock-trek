# stock-trek

A tool for running crypto bots on [stock-trek.com](https://stock-trek.com). Rust-native core.

[![crates.io](https://img.shields.io/crates/v/stock-trek)](https://crates.io/crates/stock-trek)
[![docs.rs](https://img.shields.io/docsrs/stock-trek)](https://docs.rs/stock-trek)

## Installation

Add the dependency

```sh
cargo add stock-trek
```

## Usage

Implement the `Algorithm` and `Default` traits for your algorithm and register it with the annotation `#[register_algorithm(default)]`.
A complete cost-averaging algorithm is maintained in [`examples/cost_averaging.rs`](https://github.com/Stock-Trek/stock-trek/blob/master/examples/cost_averaging.rs). It selects the CEX with the lowest latest BTC/USDT ask, and places a limit order for one satoshi at that ask.

Stock-Trek verifies code before running it and disallows certain syntax elements. To verify code locally, install the verifier [stock-trek-check](https://crates.io/crates/stock-trek-check) with

```sh
cargo install stock-trek-check
```

then run the verify command with

```sh
stock-trek-check verify --file ./path/algorithm.rs
```

## Status

This project is in early development (0.x). APIs may change.

## License

MIT
