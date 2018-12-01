
#[macro_use]
extern crate criterion;
extern crate rand_threefry;
extern crate rand_chacha;
extern crate rand_core;

use criterion::Criterion;
use rand_threefry::{ThreeFryRng,Array2x64, generate};
use rand_chacha::ChaChaRng;
use rand_core::{SeedableRng, RngCore};

const EXAMPLE_SEED1_U64: u64 = 0xdeadbeef12345678;
const EXAMPLE_SEED2_U64: u64 = 0xdecafbadbeadfeed;

fn bench_generate(c: &mut Criterion) {
    let ctr: Array2x64 = [0,1973];
    let key: Array2x64 = [EXAMPLE_SEED1_U64, EXAMPLE_SEED2_U64];
    c.bench_function("threefry_generate", |b| b.iter(|| generate(ctr, key)));
}

fn bench_u64(c: &mut Criterion) {
    let seed = [0u8; 16];
    let mut rng = ThreeFryRng::from_seed(seed);
    c.bench_function("threefry_next_u64", |b| b.iter(|| rng.next_u64()));
}

fn bench_chacha_u64(c: &mut Criterion) {
    let seed = [0u8; 32];
    let mut rng = ChaChaRng::from_seed(seed);
    c.bench_function("chacha_next_u64", |b| b.iter(|| rng.next_u64()));
}

criterion_group!(benches, bench_generate, bench_u64, bench_chacha_u64);
criterion_main!(benches);
