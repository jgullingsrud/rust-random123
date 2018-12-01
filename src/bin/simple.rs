
extern crate rand_threefry;

use rand_threefry::{Array2x64,generate};

const EXAMPLE_SEED1_U64: u64 = 0xdeadbeef12345678;
const EXAMPLE_SEED2_U64: u64 = 0xdecafbadbeadfeed;

fn main() {
    let mut ctr: Array2x64 = [0,0];
    let key: Array2x64 = [EXAMPLE_SEED1_U64, EXAMPLE_SEED2_U64];
    println!("The first few randoms with key 0x{:x} 0x{:x}", key[0], key[1]);
    for i in 0..10 {
        ctr[0] = i;
        let rand = generate(ctr, key);
        println!("ctr: {} {} threefry2x64(20, ctr, key): {:x} {:x}",
            ctr[0], ctr[1], rand[0], rand[1]);
    }
}

