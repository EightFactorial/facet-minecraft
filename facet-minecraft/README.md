# facet-minecraft

## Overview

## Usage

```rust
//! Read and write anything¹ that implements `Facet`!
//!
//! *¹ Fails on unsupported field types.

use facet::Facet;
use facet_minecraft as mc;

#[derive(Facet)]
pub struct ClientHelloPacket {
    #[facet(mc::variable)]
    pub protocol: i32,
    pub address: String,
    pub port: u16,
    pub intent: ClientIntent,
}

#[repr(u8)]
#[derive(Facet)]
pub enum ClientIntent {
    Status = 1,
    Login = 2,
    Transfer = 3,
}

// ---------------------------------------------------

let packet = ClientHelloPacket {
    protocol: 754,
    address: String::from("mc.hypixel.net"),
    port: 25565,
    intent: ClientIntent::Status,
};

match facet_minecraft::to_vec(&packet) {
    Ok(vec) => {
        assert_eq!(
            vec,
            &[
                0x00, 0x00, 0x03, 0xEA, // Protocol (754)
                0x0F, 0x6D, 0x63, 0x2E, 0x68, 0x79, 0x70, 0x69, 0x78, 0x65, 0x6C, 0x2E, 0x6E, 0x65, 0x74, // Address (15, "mc.hypixel.net")
                0x63, 0xDD, // Port (25565)
                0x01, // Intent (Status)
            ],
        )
    }
    Err(err) => panic!("Failed to serialize packet!\n{err}"),
}
```
