//! An example showing how to read the status of a Minecraft server.
//!
//! See the [Minecraft Wiki] for more information about the protocol.
//!
//! [Minecraft Wiki]: https://minecraft.wiki/w/Java_Edition_protocol/Packets
#![allow(dead_code)]

use std::{
    io::{Error, Read, Write},
    net::TcpStream,
    time::UNIX_EPOCH,
};

use facet_derive::Facet;
use facet_minecraft::{DeserializerExt, McDeserializer, McSerializer, SerializerExt};

extern crate facet_core as facet;

const SERVER_ADDRESS: &str = "mc.hypixel.net";
const SERVER_PORT: u16 = 25565;

fn main() -> Result<(), Error> {
    let mut stream = TcpStream::connect(format!("{SERVER_ADDRESS}:{SERVER_PORT}"))?;
    let mut buffer = Vec::new();

    let handshake = ServerboundHandshake::Handshake(HandshakePacket {
        // Version 1.21.6
        protocol: 771,
        address: SERVER_ADDRESS.to_string(),
        port: SERVER_PORT,
        intent: ConnectionIntent::Status,
    });

    // Send the handshake packet
    McSerializer::serialize_into(&handshake, &mut buffer).unwrap();
    send_bytes(&buffer, &mut stream)?;
    buffer.clear();

    // Send the status request
    McSerializer::serialize_into(&ServerboundStatus::StatusRequest, &mut buffer).unwrap();
    send_bytes(&buffer, &mut stream)?;
    buffer.clear();

    // Read the status response
    let response = read_bytes(&mut stream)?;
    let response = McDeserializer::deserialize::<ClientboundStatus>(&response).unwrap();
    match response {
        ClientboundStatus::PongResponse(..) => panic!("Expected to receive a status response?"),
        ClientboundStatus::StatusResponse(response) => println!("Server Status:\n{response}"),
    }

    let timestamp = UNIX_EPOCH.elapsed().unwrap().as_millis() as u64;

    // Send the ping request
    McSerializer::serialize_into(&ServerboundStatus::PingRequest(timestamp), &mut buffer).unwrap();
    send_bytes(&buffer, &mut stream)?;
    buffer.clear();

    // Read the ping response
    let response = read_bytes(&mut stream)?;
    let response = McDeserializer::deserialize::<ClientboundStatus>(&response).unwrap();
    match response {
        ClientboundStatus::StatusResponse(..) => panic!("Expected to receive a pong response?"),
        ClientboundStatus::PongResponse(response) => {
            assert_eq!(response, timestamp, "Server response did not match request?");
        }
    }

    Ok(())
}

/// Reads a packet from the stream, expecting a length prefix.
fn read_bytes(stream: &mut TcpStream) -> Result<Vec<u8>, Error> {
    const PEEK_SIZE: usize = 5;

    // Peek at the first few bytes to read the length prefix
    let mut length_buf = [0u8; PEEK_SIZE];
    stream.peek(&mut length_buf)?;
    let (length, rem) = McDeserializer.deserialize_var_usize(&length_buf).unwrap();

    // Receive the full packet including the length prefix
    let mut buffer = vec![0; PEEK_SIZE.saturating_sub(rem.len()) + length];
    stream.read_exact(&mut buffer)?;
    // Remove the length prefix
    buffer.drain(0..PEEK_SIZE.saturating_sub(rem.len()));

    Ok(buffer)
}

/// Sends a packet to the server, including a length prefix.
fn send_bytes(input: &[u8], stream: &mut TcpStream) -> Result<(), Error> {
    // Serialize the length prefix
    let mut buffer = Vec::with_capacity(5);
    McSerializer(&mut buffer).serialize_var_usize(input.len()).unwrap();

    // Write the length prefix and the data
    stream.write_all(&buffer)?;
    stream.write_all(input)?;

    Ok(())
}

// -------------------------------------------------------------------------------------------------

#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq, Facet)]
enum ServerboundHandshake {
    Handshake(HandshakePacket),
}

#[derive(Debug, Clone, PartialEq, Eq, Facet)]
struct HandshakePacket {
    #[facet(var)]
    pub protocol: i32,
    pub address: String,
    pub port: u16,
    pub intent: ConnectionIntent,
}

#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq, Facet)]
enum ConnectionIntent {
    Status = 1,
    Login = 2,
    Transfer = 3,
}

// -------------------------------------------------------------------------------------------------

#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq, Facet)]
enum ServerboundStatus {
    StatusRequest,
    PingRequest(u64),
}

#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq, Facet)]
enum ClientboundStatus {
    StatusResponse(String),
    PongResponse(u64),
}
