![License](http://img.shields.io/badge/license-MIT-lightgrey.svg)
[![Crates.io](https://img.shields.io/crates/v/paligned.svg)](https://crates.io/crates/paligned/)
[![Doc.rs](https://img.shields.io/badge/docs-docs.rs-blue.svg)](https://docs.rs/paligned/latest/paligned/)

# paligned

Simple macro for aligned printing.

```rust
use paligned::paligned;

paligned! ([
    ["expression","value", "description"],
    ["1 + 1 =", 1 + 1],
    ["foo:", @debug foo, "`@debug` example"],
]);
// expression value          description
// 1 + 1 =    2
// foo:       Foo { val: 1 } `@debug` example
```

The goal of this crate is to provide a simple, zero-configuration solution for printing several rows with aligned columns. Basically, an alternative to:

```rust
println!("expression value  description");
println!("1 + 1 = {}", 1 + 1);
println!("foo:    {:?}", foo);
```

This macro is minimal on purpose, if you need fancier printing, then (at the cost of extra verbosity) I would recommend:

- [`prettytable-rs`](https://crates.io/crates/prettytable-rs)
- [`tabled`](https://crates.io/crates/tabled)
