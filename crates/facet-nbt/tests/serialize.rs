//! Test deserializing and re-serializing example [`Nbt`] data.

use std::io::Read;

use facet_nbt::{NbtMap, tape::NbtTape};

#[cfg(debug_assertions)]
fn init_trace() {
    let _ = tracing_subscriber::fmt().with_max_level(tracing::Level::TRACE).try_init();
}

#[test]
fn empty_unnamed() {
    static DATA: &[u8] = &[10, 0];

    #[cfg(debug_assertions)]
    init_trace();

    let mut tape = NbtTape::unnamed_from_slice(DATA).unwrap();
    let map = NbtMap::try_from_tape(&tape).unwrap();

    println!("{tape:#?}");
    println!("{map:#?}");

    assert!(tape.trim().is_empty());
}

#[test]
fn empty_named() {
    static DATA: &[u8] = &[10, 0, 0, 0];

    #[cfg(debug_assertions)]
    init_trace();

    let mut tape = NbtTape::named_from_slice(DATA).unwrap();
    let map = NbtMap::try_from_tape(&tape).unwrap();

    println!("{tape:#?}");
    println!("{map:#?}");

    assert!(tape.trim().is_empty());
}

// -------------------------------------------------------------------------------------------------

#[test]
fn hello_world() {
    static DATA: &[u8] = include_bytes!("data/hello_world.nbt");

    #[cfg(debug_assertions)]
    init_trace();

    let mut tape = NbtTape::named_from_slice(DATA).unwrap();
    let map = NbtMap::try_from_tape(&tape).unwrap();

    println!("{tape:#?}");
    println!("{map:#?}");

    assert!(tape.trim().is_empty());
}

#[test]
fn complex_player() {
    static DATA: &[u8] = include_bytes!("data/complex_player.nbt.gz");

    #[cfg(debug_assertions)]
    init_trace();

    let mut decoder = flate2::read::GzDecoder::new(DATA);
    let mut uncompressed = Vec::new();
    decoder.read_to_end(&mut uncompressed).unwrap();

    let mut tape = NbtTape::named_from_slice(&uncompressed).unwrap();
    // let map = NbtMap::try_from_tape(&tape).unwrap();

    println!("{tape:#?}");
    // println!("{map:#?}");

    assert!(tape.trim().is_empty());
}

#[test]
fn hypixel() {
    static DATA: &[u8] = include_bytes!("data/hypixel.nbt");

    #[cfg(debug_assertions)]
    init_trace();

    let mut tape = NbtTape::named_from_slice(DATA).unwrap();
    // let map = NbtMap::try_from_tape(&tape).unwrap();

    println!("{tape:#?}");
    // println!("{map:#?}");

    assert!(tape.trim().is_empty());
}

#[test]
fn inttest1023() {
    static DATA: &[u8] = include_bytes!("data/inttest1023.nbt");

    #[cfg(debug_assertions)]
    init_trace();

    let mut tape = NbtTape::named_from_slice(DATA).unwrap();
    let map = NbtMap::try_from_tape(&tape).unwrap();

    println!("{tape:#?}");
    println!("{map:#?}");

    assert!(tape.trim().is_empty());
}
