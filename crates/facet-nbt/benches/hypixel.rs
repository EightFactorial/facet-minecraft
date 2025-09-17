//! Benchmarks for de/serializing `hypixel.nbt`

fn main() { divan::main() }

/// A sample of NBT data from Hypixel.
static HYPIXEL_NBT: &[u8] = include_bytes!("../tests/data/hypixel.nbt");

#[divan::bench_group]
mod serialize {
    // #[divan::bench]
    // fn facet_nbt(mut b: divan::Bencher) {
    //     let (nbt, _) = NamedNbt::read_from(super::HYPIXEL_NBT).unwrap();
    //     let mut buffer = Vec::with_capacity(super::HYPIXEL_NBT.len());

    //     b = b.counter(divan::counter::BytesCount::of_slice(super::HYPIXEL_NBT));
    //     b.bench_local(|| divan::black_box_drop(nbt.write_to(&mut buffer)));
    // }

    // #[divan::bench]
    // fn facet_minecraft(mut b: divan::Bencher) {
    //     let nbt =
    // facet_minecraft::deserialize::<NamedNbt>(super::HYPIXEL_NBT).unwrap();
    //     let mut buffer = Vec::with_capacity(super::HYPIXEL_NBT.len());

    //     b = b.counter(divan::counter::BytesCount::of_slice(super::HYPIXEL_NBT));
    //     b.bench_local(||
    // divan::black_box_drop(facet_minecraft::serialize(&nbt, &mut buffer)));
    // }
}

#[divan::bench_group]
mod deserialize {
    use facet_nbt::tape::NbtTape;

    #[divan::bench]
    fn facet_nbt(mut b: divan::Bencher) {
        b = b.counter(divan::counter::BytesCount::of_slice(super::HYPIXEL_NBT));
        b.bench(|| divan::black_box_drop(NbtTape::named_from_slice(super::HYPIXEL_NBT).unwrap()));
    }

    // #[divan::bench]
    // fn facet_minecraft(mut b: divan::Bencher) {
    //     b = b.counter(divan::counter::BytesCount::of_slice(super::HYPIXEL_NBT));
    //     b.bench(|| {
    //         divan::black_box_drop(facet_minecraft::deserialize::<NamedNbt>(super::HYPIXEL_NBT));
    //     });
    // }
}
