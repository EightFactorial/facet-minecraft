<h1 align="center">facet-minecraft</h1>
<p align="center">facet-based serialization and deserialization crates that support parts of the Minecraft protocol</p>

## Overview

## Usage

```rust
//! Read and write anything¹ that implements `Facet`!
//!
//! *¹ Fails on unsupported field types.

use facet::Facet;
#[cfg(feature = "uuid")]
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Facet)]
pub struct PlayerInfo {
    pub name: String,
    #[cfg(feature = "uuid")]
    pub uuid: Uuid,
    pub properties: Vec<PlayerProperty>,
}

#[derive(Debug, Clone, PartialEq, Eq, Facet)]
pub struct PlayerProperty {
    pub name: String,
    pub value: String,
    pub signature: Option<String>,
}

// Create an example player profile
let profile = PlayerInfo {
    name:  "jeb_".into(),
    #[cfg(feature = "uuid")]
    uuid: Uuid::parse_str("853c80ef-3c37-49fd-aa49-938b674adae6").unwrap(),
    properties: vec![
        PlayerProperty { name: "textures".into(), value: "pretend_base64".into(), signature: None },
    ],
};

// Write the profile to a buffer
let mut buffer = Vec::<u8>::new();
facet_minecraft::serialize(&profile, &mut buffer).unwrap();

// Read the profile back from the buffer
match facet_minecraft::deserialize::<PlayerInfo>(&buffer) {
    Ok(read) => pretty_assertions::assert_eq!(read, profile),
    #[cfg(feature = "rich-diagnostics")]
    Err(err) => panic!("{}", err.as_report()),
    #[cfg(not(feature = "rich-diagnostics"))]
    Err(err) => panic!("{err}"),
}
```

```rust
//! TODO: (S)NBT Example
```

## Release Checklist

- [ ] Protocol
  - [ ] Documentation
  - [x] Serialization
    - [ ] Assert trait
    - [ ] Rich diagnostics
  - [x] Deserialization
    - [ ] Assert trait
    - [x] Rich diagnostics
- [ ] NBT
  - [ ] Documentation
  - [x] Serialization
    - [ ] Rich diagnostics
  - [x] Deserialization
    - [ ] Zero-copy
    - [ ] Rich diagnostics
  - [x] Protocol support
  - [ ] Struct-to-NBT conversion (future update?)
    - [ ] Assert trait
    - [ ] Rich diagnostics
  - [ ] NBT-to-struct conversion (future update?)
    - [ ] Assert trait
    - [ ] Rich diagnostics
- [ ] SNBT
  - [ ] Documentation
  - [x] Serialization
    - [ ] Rich diagnostics
  - [x] Deserialization
    - [ ] Rich diagnostics
  - [x] NBT-to-SNBT conversion
    - [ ] Rich diagnostics
  - [ ] SNBT-to-NBT conversion
    - [ ] Rich diagnostics

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
