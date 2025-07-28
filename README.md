<h1 align="center">facet-minecraft</h1>
<p align="center">Facet serialization/deserialization crates that support parts of the Minecraft protocol</p>

## Overview

## Usage

```rust
use facet::Facet;

/// A player struct
#[derive(Facet)]
struct MyPlayer {
    name: String,
    #[facet(var)]
    health: u32,
}

// Create a `MyPlayer` instance
let player = MyPlayer {
    name: String::from("Steve"),
    health: 20,
};

// Serialize the player into the buffer
let mut buffer = Vec::new();
facet_minecraft::serialize(&player, &mut buffer).unwrap();

// The buffer now contains the data in the correct format
assert_eq!(buffer, vec![5u8, b'S', b't', b'e', b'v', b'e', 20u8]);
```

## Todo List

- [x] Deserialization
- [x] Serialization
- [ ] Protocol Assertions
  - Works in some cases but not all

## License

Licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
