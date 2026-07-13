# Rinia

[![Tag](https://img.shields.io/github/v/tag/tailflare/rinia)](https://github.com/tailflare/rinia/tags)
[![Crates.io Version](https://img.shields.io/crates/v/rinia)](https://crates.io/crates/rinia)
[![docs.rs](https://img.shields.io/docsrs/rinia)](https://docs.rs/rinia)
[![Main CI Build Status](https://img.shields.io/github/actions/workflow/status/tailflare/rinia/ci.yml?label=main%20build)](https://github.com/tailflare/rinia/actions/workflows/ci.yml)
[![License](https://img.shields.io/crates/l/rinia)](#license)

Stable numeric foundations for graphics, animation, and simulation.

Rinia provides a lightweight, portable foundation of numeric abstractions and mathematical primitives for Rust applications. It defines consistent scalar models, operations, conversions, and representations for building reliable graphics, animation, and simulation systems.

Rinia separates mathematical structures from their underlying numeric representations, allowing mathematical primitives to be expressed generically across different numeric types. By providing shared scalar traits, numeric operations, and predictable conversion behavior, Rinia enables these primitives to be reused across engines, tools, runtimes, and processing pipelines without being tied to a single scalar type such as `f32`.

Designed for portability, Rinia supports `no_std` and `no_alloc` environments while providing consistent numeric behavior and mathematical operations across platforms.

Rinia is designed for projects that need stable mathematical types and predictable numeric behavior across system boundaries without depending on a larger engine-specific math stack.

Rinia is not intended to replace specialized math libraries where maximum performance or platform-specific optimizations are the primary goal. Instead, it provides a reliable foundation layer for applications that prioritize portability, interoperability, and consistency.

## Status

⚠️ Experimental / Work in Progress

The API is still evolving and may change significantly.

## License

The rinia project is licensed under either the Apache License, Version 2.0 or the MIT license, at your option.

See [LICENSE-APACHE](./LICENSE-APACHE) and [LICENSE-MIT](./LICENSE-MIT) for details.
