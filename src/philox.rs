
#[inline]
fn mul32(a: u32, b:u32) -> (u32, u32) {
    let prod = (a as u64).wrapping_mul(b as u64);
    ((prod >> 32) as u32, prod as u32)
}

#[inline]
fn mul64(a: u64, b:u64) -> (u64, u64) {
    let prod = (a as u128).wrapping_mul(b as u128);
    ((prod >> 64) as u64, prod as u64)
}


// multipliers and Weyl constants
const PHILOX_M2X64_0: u64 = 0xD2B74407B1CE6E93;
const PHILOX_M4X64_0: u64 = 0xD2E7470EE14C6C93;
const PHILOX_M4X64_1: u64 = 0xCA5A826395121157;
const PHILOX_W64_0: u64 = 0x9E3779B97F4A7C15;   // golden ratio
const PHILOX_W64_1: u64 = 0xBB67AE8584CAA73B;   // sqrt(3)-1

const PHILOX_M2X32_0:u32 = 0xd256d193;
const PHILOX_M4X32_0:u32 = 0xD2511F53;
const PHILOX_M4X32_1:u32 = 0xCD9E8D57;
const PHILOX_W32_0:u32 = 0x9E3779B9;
const PHILOX_W32_1:u32 = 0xBB67AE85;

pub type Array1x32 = [u32; 1];
pub type Array2x32 = [u32; 2];
pub type Array4x32 = [u32; 4];
pub type Array1x64 = [u64; 1];
pub type Array2x64 = [u64; 2];
pub type Array4x64 = [u64; 4];

fn philox_2x32round(ctr: Array2x32, key: Array1x32) -> Array2x32 {
    let (hi, lo) = mul32(PHILOX_M2X32_0, ctr[0]);
    [hi ^ key[0]^ctr[1], lo]
}

fn philox_2x64round(ctr: Array2x64, key: Array1x64) -> Array2x64 {
    let (hi, lo) = mul64(PHILOX_M2X64_0, ctr[0]);
    [hi ^ key[0]^ctr[1], lo]
}

fn philox_4x32round(ctr: Array4x32, key: Array2x32) -> Array4x32 {
    let (hi0, lo0) = mul32(PHILOX_M4X32_0, ctr[0]);
    let (hi1, lo1) = mul32(PHILOX_M4X32_1, ctr[2]);
    [hi1^ctr[1]^key[0], lo1, hi0^ctr[3]^key[1], lo0]
}

fn philox_4x64round(ctr: Array4x64, key: Array2x64) -> Array4x64 {
    let (hi0, lo0) = mul64(PHILOX_M4X64_0, ctr[0]);
    let (hi1, lo1) = mul64(PHILOX_M4X64_1, ctr[2]);
    [hi1^ctr[1]^key[0], lo1, hi0^ctr[3]^key[1], lo0]
}

fn philox_2x32key(key: Array2x32) -> Array2x32 {
    [key[0].wrapping_add(PHILOX_W32_0), key[1].wrapping_add(PHILOX_W32_1)]
}

pub fn philox_4x32(ctr: Array4x32, key: Array2x32) -> Array4x32 {
                                   let ctr = philox_4x32round(ctr, key);    // 0
    let key = philox_2x32key(key); let ctr = philox_4x32round(ctr, key);    // 1
    let key = philox_2x32key(key); let ctr = philox_4x32round(ctr, key);    // 2
    let key = philox_2x32key(key); let ctr = philox_4x32round(ctr, key);    // 3
    let key = philox_2x32key(key); let ctr = philox_4x32round(ctr, key);    // 4
    let key = philox_2x32key(key); let ctr = philox_4x32round(ctr, key);    // 5
    let key = philox_2x32key(key); let ctr = philox_4x32round(ctr, key);    // 6
    let key = philox_2x32key(key); let ctr = philox_4x32round(ctr, key);    // 7
    let key = philox_2x32key(key); let ctr = philox_4x32round(ctr, key);    // 8
    let key = philox_2x32key(key); let ctr = philox_4x32round(ctr, key);    // 9
    ctr
}

#[cfg(test)]
mod tests {
    const TEST_VEC_1: [u32; 40] = [
        0xcc7d356a, 0x5e7dedd7, 0x76798bc3, 0x6c05818c,
        0x4d7d84fc, 0x44ea4626, 0x26680a11, 0xc5c86681,
        0x5344ffa0, 0xa0300aea, 0x650c4611, 0xf274539d,
        0x99b25360, 0x9316678 , 0xd791ce76, 0xb12c3349,
        0xa65ceb5 , 0x9514aef7, 0xe5528b10, 0x7e6416b4,
        0x36f3b5f , 0xa78d244b, 0x192afe87, 0xfa93201,
        0x45d59db0, 0xd93533dd, 0x150a6435, 0x88f8a2e8,
        0x1882b35 , 0xe365ff23, 0x4e06c6cf, 0x4a2d3133,
        0x1ea732e6, 0x8835f7fd, 0x20219e72, 0xfde01b3a,
        0xc1c5424f, 0x1591eacd, 0x90e83125, 0x471a8bf4,
    ];
    const SEED1: u32 = 0x11111111;
    const SEED2: u32 = 0x22222222;

    use super::{Array2x32, Array4x32, philox_4x32};
    //use rand_core::{RngCore, SeedableRng};

    #[test]
    fn exact_values() {
        let mut ctr: Array4x32 = [0,0,0,0];
        let key: Array2x32 = [SEED1, SEED2];
        for i in 0..10 {
            ctr[0] = i;
            let x = philox_4x32(ctr, key);
            let i0 = (4*i+0) as usize;
            let i1 = (4*i+1) as usize;
            let i2 = (4*i+2) as usize;
            let i3 = (4*i+3) as usize;
            assert_eq!(x[0], TEST_VEC_1[i0]);
            assert_eq!(x[1], TEST_VEC_1[i1]);
            assert_eq!(x[2], TEST_VEC_1[i2]);
            assert_eq!(x[3], TEST_VEC_1[i3]);
        }
    }

//    #[test]
//    fn next_u64() {
//        let mut rng = ThreeFryRng::seed_from_u64(0);
//        rng.set_key(SEED1_U64, SEED2_U64);
//        for i in 0..20 {
//            assert_eq!(rng.next_u64(), TEST_VEC_1[i]);
//        }
//    }
//
//    #[test]
//    fn seedable() {
//        let mut rng = ThreeFryRng::seed_from_u64(42);
//        assert_eq!(rng.next_u64(), 391376552519608501);
//    }
}
