extern crate rand_core;

use rand_core::{RngCore, Error, impls};

pub type Array2x64 = [u64; 2];

pub struct ThreeFryRng {
    pub ctr: Array2x64,
    pub key: Array2x64,
    // TODO: implement and benchmark a buffer so we don't
    // burn half our bits.
}

impl RngCore for ThreeFryRng {
    fn next_u64(&mut self) -> u64 {
        let rand: u64 = generate(self.ctr, self.key)[0];
        self.ctr[0] = self.ctr[0].wrapping_add(1);
        if self.ctr[0] == 0 {
            self.ctr[1] = self.ctr[1].wrapping_add(1);
        }
        rand
    }
    fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        impls::fill_bytes_via_next(self, dest)
    }
    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
        Ok(self.fill_bytes(dest))
    }
}

// TODO: use std::fmt; impl fmt::Debug

const SKEIN_HI: u64 = 0x1BD11BDA;
const SKEIN_LO: u64 = 0xA9FC1A22;
const SKEIN_PARITY: u64 = SKEIN_LO + (SKEIN_HI << 32);

const R_64X2_0_0: u32 = 16;
const R_64X2_1_0: u32 = 42;
const R_64X2_2_0: u32 = 12;
const R_64X2_3_0: u32 = 31;
const R_64X2_4_0: u32 = 16;
const R_64X2_5_0: u32 = 32;
const R_64X2_6_0: u32 = 24;
const R_64X2_7_0: u32 = 21;

macro_rules! round1 {
    ($x: expr) => {{
        $x[0]=$x[0].wrapping_add($x[1]); $x[1]=$x[1].rotate_left(R_64X2_0_0); $x[1]^=$x[0];
        $x[0]=$x[0].wrapping_add($x[1]); $x[1]=$x[1].rotate_left(R_64X2_1_0); $x[1]^=$x[0];
        $x[0]=$x[0].wrapping_add($x[1]); $x[1]=$x[1].rotate_left(R_64X2_2_0); $x[1]^=$x[0];
        $x[0]=$x[0].wrapping_add($x[1]); $x[1]=$x[1].rotate_left(R_64X2_3_0); $x[1]^=$x[0];
    }}
}

macro_rules! round2 {
    ($x: expr) => {{
        $x[0]=$x[0].wrapping_add($x[1]); $x[1]=$x[1].rotate_left(R_64X2_4_0); $x[1]^=$x[0];
        $x[0]=$x[0].wrapping_add($x[1]); $x[1]=$x[1].rotate_left(R_64X2_5_0); $x[1]^=$x[0];
        $x[0]=$x[0].wrapping_add($x[1]); $x[1]=$x[1].rotate_left(R_64X2_6_0); $x[1]^=$x[0];
        $x[0]=$x[0].wrapping_add($x[1]); $x[1]=$x[1].rotate_left(R_64X2_7_0); $x[1]^=$x[0];
    }}
}

macro_rules! sbox {
    ($x: expr, $ks: expr, $k: expr) => {{
        $x[0]=$x[0].wrapping_add($ks[($k+0)%3]);
        $x[1]=$x[1].wrapping_add($ks[($k+1)%3]);
        $x[1]=$x[1].wrapping_add($k);
    }}
}

pub fn generate(ctr: Array2x64, key: Array2x64) -> Array2x64 {
    let mut x: Array2x64 = [0, 0];
    let mut ks: [u64; 3] = [0, 0, SKEIN_PARITY];
    for i in 0..2 {
        ks[i] = key[i];
        x[i] = ctr[i];
        ks[2] ^= key[i];
    }

    sbox!(x, ks, 0);

    round1!(x);
    sbox!(x, ks, 1);

    round2!(x);
    sbox!(x, ks, 2);

    round1!(x);
    sbox!(x, ks, 3);

    round2!(x);
    sbox!(x, ks, 4);

    round1!(x);
    sbox!(x, ks, 5);

    x
}

#[cfg(test)]
mod tests {
    const TEST_VEC_1: [u64; 20] = [
        0x3c956fe5e3e09745,  0x911f953cce0c0674,
        0xbf307d9a09b8e517,  0x21255fa6b494c50e,
        0x36767323a0f90211,  0xb5912b450fc89b38,
        0x2d5703abf89c5424,  0xa0c7471ae60d0622,
        0x24a9f70a44338b6d,  0xd1396ce94674b224,
        0xa30049ea40bfed1 ,  0xec7400474a7fe8f0,
        0x37fa17b7c8b37514,  0x7d38e5e4f0eb3a1,
        0x1e176ae521a2c8c6,  0x88f7022bae92e50d,
        0x19ce7fbd095eb0f8,  0x65eaf3fc558b735c,
        0xfad725f62c08e780,  0x1e91764c67bc64e6,
    ];
    const EXAMPLE_SEED1_U64: u64 = 0xdeadbeef12345678;
    const EXAMPLE_SEED2_U64: u64 = 0xdecafbadbeadfeed;

    use super::{ThreeFryRng, Array2x64, generate};
    use rand_core::RngCore;

    #[test]
    fn exact_values() {
        let mut ctr: Array2x64 = [0,0];
        let key: Array2x64 = [EXAMPLE_SEED1_U64, EXAMPLE_SEED2_U64];
        for i in 0..10 {
            ctr[0] = i;
            let rand = generate(ctr, key);
            let i0 = (2*i+0) as usize;
            let i1 = (2*i+1) as usize;
            assert_eq!(rand[0], TEST_VEC_1[i0]);
            assert_eq!(rand[1], TEST_VEC_1[i1]);
        }
    }

    #[test]
    fn next_u64() {
        let ctr: Array2x64 = [0,0];
        let key: Array2x64 = [EXAMPLE_SEED1_U64, EXAMPLE_SEED2_U64];
        let mut rng = ThreeFryRng { ctr, key };
        for i in 0..10 {
            assert_eq!(rng.next_u64(), TEST_VEC_1[2*i]);
        }
    }
}

