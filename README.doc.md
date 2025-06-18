# TODO: Docs

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
