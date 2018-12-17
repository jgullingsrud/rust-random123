
#[macro_use]
extern crate criterion;
extern crate random123;
extern crate rand_chacha;
extern crate rand_core;

use criterion::Criterion;
use random123::threefry::{ThreeFryRng,Array2x64, rand};
use random123::philox::{philox_4x32,Array2x32, Array4x32};
use rand_chacha::ChaChaRng;
use rand_core::{SeedableRng, RngCore};

const EXAMPLE_SEED1_U64: u64 = 0xdeadbeef12345678;
const EXAMPLE_SEED2_U64: u64 = 0xdecafbadbeadfeed;

const SEED1: u32 = 0x11111111;
const SEED2: u32 = 0x22222222;

fn bench_rand(c: &mut Criterion) {
    let ctr: Array2x64 = [0,1973];
    let key: Array2x64 = [EXAMPLE_SEED1_U64, EXAMPLE_SEED2_U64];
    let mut x: Array2x64 = [0,0];
    c.bench_function("threefry_generate", |b| b.iter(|| rand(ctr, key, &mut x)));
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

fn bench_philox_4x32(c: &mut Criterion) {
    let ctr: Array4x32 = [0,1,2,3];
    let key: Array2x32 = [SEED1, SEED2];
    c.bench_function("philox4x32", |b| b.iter(|| philox_4x32(ctr, key)));
}

criterion_group!(benches, bench_rand, bench_u64, bench_chacha_u64, bench_philox_4x32);
criterion_main!(benches);
