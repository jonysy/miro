mīrō
====

[![Join the chat at https://gitter.im/lychee-eng/miro](https://badges.gitter.im/lychee-eng/miro.svg)](https://gitter.im/lychee-eng/miro?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)
[![Build Status](https://travis-ci.org/lychee-eng/miro.svg?branch=master)](https://travis-ci.org/lychee-eng/miro)

A modern, open, high-performance, ([soon to be](/../../issues/5)) GPU-accelerated computer vision 
library providing state-of-the-art implementations and performance. See the [issues] page for a list 
of features planned or in development.

## Getting Started

### Installation

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

### Contributing

1) Fork

2) Optionally create a new branch (e.g., `git checkout -b my-new-feature`) (*recommended if you're 
making multiple changes)
    
3) Commit your changes (*[5 useful tips for a better commit message])
    
4) Push (e.g., `git push origin my-new-feature`)

5) Create a new pull request

### Contribution

This project is a labor of love, as well as a community effort - new contributors are welcome.
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the 
work by you shall be dual licensed as below, without any additional terms or conditions.

## Community 

People are expected to follow a code of conduct similar to that of 
the [Rust organization][Rust Code of Conduct] when discussing mīrō on the Github page, Gitter 
channel, or other venues.

## License

Dual licensed under
  * Apache License, Version 2.0 ([LICENSE-APACHE][apache] or http://www.apache.org/licenses/LICENSE-2.0)
  * MIT license ([LICENSE-MIT][mit] or http://opensource.org/licenses/MIT)

## License

Dual licensed under
  * Apache License, Version 2.0 ([LICENSE-APACHE][apache] or http://www.apache.org/licenses/LICENSE-2.0)
  * MIT license ([LICENSE-MIT][mit] or http://opensource.org/licenses/MIT)

[issues]: /../../issues
[rustup]: https://www.rustup.rs
[apache]: ../../../license/blob/master/LICENSE-APACHE
[mit]: ../../../license/blob/master/LICENSE-MIT
[5 useful tips for a better commit message]: https://robots.thoughtbot.com/5-useful-tips-for-a-better-commit-message
[Rust Code of Conduct]: https://www.rust-lang.org/en-US/conduct.html
