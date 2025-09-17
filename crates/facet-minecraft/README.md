# facet-minecraft

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
