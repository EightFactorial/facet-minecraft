//! Benchmarks for de/serializing `complex_player.nbt`

fn main() { divan::main() }

/// A sample of NBT data.
static COMPLEX_PLAYER_NBT: &[u8] = include_bytes!("../tests/data/complex_player.nbt.gz");

// #[divan::bench_group]
// mod serialize {
//     use std::io::Read;

//     use facet_nbt::nbt::NamedNbt;

//     #[divan::bench(sample_size = 5000, sample_count = 250)]
//     fn facet_nbt(mut b: divan::Bencher) {
//         let mut decoder =
// flate2::read::GzDecoder::new(super::COMPLEX_PLAYER_NBT);         let mut
// uncompressed = Vec::new();         decoder.read_to_end(&mut
// uncompressed).unwrap();

//         let (nbt, _) = NamedNbt::read_from(&uncompressed).unwrap();
//         let mut buffer = Vec::with_capacity(uncompressed.len());

//         b = b.counter(divan::counter::BytesCount::of_slice(&uncompressed));
//         b.bench_local(|| divan::black_box_drop(nbt.write_to(&mut buffer)));
//     }

//     #[divan::bench(sample_size = 5000, sample_count = 250)]
//     fn facet_minecraft(mut b: divan::Bencher) {
//         let mut decoder =
// flate2::read::GzDecoder::new(super::COMPLEX_PLAYER_NBT);         let mut
// uncompressed = Vec::new();         decoder.read_to_end(&mut
// uncompressed).unwrap();

//         let nbt =
// facet_minecraft::deserialize::<NamedNbt>(&uncompressed).unwrap();         let
// mut buffer = Vec::with_capacity(uncompressed.len());

//         b = b.counter(divan::counter::BytesCount::of_slice(&uncompressed));
//         b.bench_local(||
// divan::black_box_drop(facet_minecraft::serialize(&nbt, &mut buffer)));     }
// }

#[divan::bench_group]
mod deserialize {
    use std::io::Read;

    use facet_nbt::tape::NbtTape;

    #[divan::bench]
    fn facet_nbt(mut b: divan::Bencher) {
        let mut decoder = flate2::read::GzDecoder::new(super::COMPLEX_PLAYER_NBT);
        let mut uncompressed = Vec::new();
        decoder.read_to_end(&mut uncompressed).unwrap();

        b = b.counter(divan::counter::BytesCount::of_slice(&uncompressed));
        b.bench(|| divan::black_box_drop(NbtTape::named_from_slice(&uncompressed).unwrap()));
    }

    // #[divan::bench(sample_size = 5000, sample_count = 250)]
    // fn facet_minecraft(mut b: divan::Bencher) {
    //     let mut decoder =
    // flate2::read::GzDecoder::new(super::COMPLEX_PLAYER_NBT);     let mut
    // uncompressed = Vec::new();     decoder.read_to_end(&mut
    // uncompressed).unwrap();

    //     b = b.counter(divan::counter::BytesCount::of_slice(&uncompressed));
    //     b.bench(||
    // divan::black_box_drop(facet_minecraft::deserialize::<NamedNbt>(&
    // uncompressed))); }
}
