# stock-trek-macros

Procedural macros for [stock-trek](https://github.com/Stock-Trek/stock-trek).

[![crates.io](https://img.shields.io/crates/v/stock-trek-macros)](https://crates.io/crates/stock-trek-macros)
[![docs.rs](https://img.shields.io/docsrs/stock-trek-macros)](https://docs.rs/stock-trek-macros)

This crate provides `#[register_algorithm]`, which registers implementations of traits used by stock-trek.
The macro is re-exported by the main crate, so algorithms normally import it through `stock_trek::prelude::*` or `stock_trek::register_algorithm` rather than depending on this crate directly.

## Usage

Implement the trait on a concrete type, derive or implement `Default`, and apply the attribute to the implementation:

```rust
use stock_trek::prelude::*;

#[derive(Default)]
struct MyAlgorithm;

#[register_algorithm]
impl Algorithm for MyAlgorithm {
    fn name(&self) -> &str {
        "MyAlgorithm"
    }

    // Implement the remaining Algorithm methods here.
}
```

A complete example is available in [`examples/cost_averaging.rs`](https://github.com/Stock-Trek/stock-trek/blob/master/examples/cost_averaging.rs).

## Behaviour

- The attribute takes no arguments and must be placed on a trait implementation.
- The implementing type must implement `Default`; registrations construct the type through `Default` when they are read.
- Generic implementations, inverted implementations, and implementations on non-path types are rejected at compile time.
- Registrations are collected through target-specific module initialisers, so they are available before the host them on native and WebAssembly targets.

Registered implementations can be read with `RegisteredImpl::all`, or with `RegisteredImpl::algorithms` for the `Algorithm` trait.

## License

MIT
