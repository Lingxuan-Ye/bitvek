# bitvek

[![Crates.io](https://img.shields.io/crates/v/bitvek.svg)](https://crates.io/crates/bitvek)
[![Documentation](https://docs.rs/bitvek/badge.svg)](https://docs.rs/bitvek)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

Say, we have a bit vector —

it's nothing better than a `Vec<bool>`, but …

what if we implement it,

and save some poor bits of memory?

## Quick Start

The following vector only takes **two** bytes of the heap memory!

```rust
use bitvek::bitvec;

let vec = bitvec![
    true, true, true, true, false, false, false, false,
    false, false, false, false, true, true, true, true,
];
```
Find it cumbersome? Try this:

```rust
// requires the total number of bits to be a multiple of 8
let vec = bitvec![0b1111_0000, 0b0000_1111];
```
