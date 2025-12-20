//! TODO

fn main() {}

// -------------------------------------------------------------------------------------------------

use facet::Facet;
use facet_minecraft as mc;

#[derive(Facet)]
struct Test {
    #[facet(mc::variable)]
    field_a: u32,
    #[facet(mc::serialize = serialize_fn, mc::deserialize = deserialize_fn)]
    field_b: u32,
}

fn serialize_fn() {}

fn deserialize_fn() {}
