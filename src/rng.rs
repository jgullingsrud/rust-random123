use core::fmt;
use rand_core::{RngCore, SeedableRng, Error};
use rand_core::block::{BlockRngCore, BlockRng, BlockRng64};

use super::philox::{Philox2x32,  Philox2x64,  Philox4x32,  Philox4x64};

macro_rules! impl_rng {
    ($t: ty, $n:expr, $i: ty, $b: expr, $block: ident, $rng: path) => {

        impl BlockRngCore for $t {
            type Item = $i;
            type Results = [$i; $n];

            fn generate(&mut self, results: &mut Self::Results) {
                *results = self.next();
            }
        }

        impl SeedableRng for $t {
            type Seed = [u8; $b];

            fn from_seed(seed: Self::Seed) -> Self {
                <$t>::from_seed(seed)
            }
        }

        impl fmt::Debug for $t {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{} {{}}", stringify!($t))
            }
        }

        impl SeedableRng for $rng {
            type Seed = <$t as SeedableRng>::Seed;

            fn from_seed(seed: Self::Seed) -> Self {
                $rng($block::<$t>::from_seed(seed))
            }

            fn from_rng<R: RngCore>(rng: R) -> Result<Self, Error> {
                $block::<$t>::from_rng(rng).map($rng)
            }
        }


    }
}

#[derive(Clone, Debug)]
pub struct Philox2x32Rng(BlockRng<Philox2x32>);
#[derive(Clone, Debug)]
pub struct Philox2x64Rng(BlockRng64<Philox2x64>);
#[derive(Clone, Debug)]
pub struct Philox4x32Rng(BlockRng<Philox4x32>);
#[derive(Clone, Debug)]
pub struct Philox4x64Rng(BlockRng64<Philox4x64>);

impl_rng!(Philox2x32, 2, u32, 4, BlockRng,   Philox2x32Rng);
impl_rng!(Philox2x64, 2, u64, 8, BlockRng64, Philox2x64Rng);
impl_rng!(Philox4x32, 4, u32, 8, BlockRng,   Philox4x32Rng);
impl_rng!(Philox4x64, 4, u64,16, BlockRng64, Philox4x64Rng);

