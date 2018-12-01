
#[macro_use]
extern crate criterion;
extern crate rand_threefry;

use criterion::Criterion;
use rand_threefry::{Array2x64, generate};

const EXAMPLE_SEED1_U64: u64 = 0xdeadbeef12345678;
const EXAMPLE_SEED2_U64: u64 = 0xdecafbadbeadfeed;

fn bench_generate(c: &mut Criterion) {
    let ctr: Array2x64 = [0,1973];
    let key: Array2x64 = [EXAMPLE_SEED1_U64, EXAMPLE_SEED2_U64];
    c.bench_function("generate", |b| b.iter(|| generate(ctr, key)));
}

criterion_group!(benches, bench_generate);
criterion_main!(benches);
