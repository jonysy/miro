A modern, open, high-performance computer vision library.

[![Build Status](https://travis-ci.org/lychee-eng/miro.svg?branch=master)](https://travis-ci.org/lychee-eng/miro)
[![License](https://img.shields.io/crates/l/miro.svg)](LICENSE)
[![Join the chat at https://gitter.im/lychee-eng/miro](https://badges.gitter.im/lychee-eng/miro.svg)](https://gitter.im/lychee-eng/miro?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)

## Introduction

A modern, (soon to be) GPU-accelerated computer vision library providing state-of-the-art implementations and performance.

## Philosophy

* __Application Driven__:</br>
... For a more research-driven approach, have a look at [mirage](/../../../mirage) and [illusions](/../../../illusions).

* __Well-Written Documentation__:</br>
Includes references, pseudo-code, brief descriptions illustrating the concept of the algorithms and data-structures, ..

* __Modular__:</br>
...The crate itself is made up of [μcrates](../../tree/master/crates)...

## Ecosystem

See the [issues](/../../issues) page for a list of features planned or in development.

μcrates:

- [classification](/../../issues/18)
- [detection](/../../issues/21)
- [euclidean geometry](/../../issues/19)
- [motion](/../../issues/7)
- [tracking](/../../issues/11)

## Quick-start

### Installing Rust

I recommend you install [rustup][rustup] and then run the following command to use 
the Rust nightly version:

```sh
$ rustup install nightly # or rustup update nightly
```

Although you can run a command from the nightly toolchain without switching to 
nightly (`$ rustup run nightly [command]`), you should switch to nightly:

```sh
$ rustup default nightly
```

### Usage

```toml
[dependencies.miro]
version = ..
```

```rust
extern crate miro;

fn main() {
	...
}
```

### Tests

```
$ cargo test -p miro-euclidean
```

## License

Dual licensed under
  * Apache License, Version 2.0 ([LICENSE-APACHE][apache] or http://www.apache.org/licenses/LICENSE-2.0)
  * MIT license ([LICENSE-MIT][mit] or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the 
work by you shall be dual licensed as above, without any additional terms or conditions.

[rustup]: https://www.rustup.rs
[apache]: ../../../license/blob/master/LICENSE-APACHE
[mit]: ../../../license/blob/master/LICENSE-MIT