#![no_std]

pub mod hash;

use crate::hash::seed_hash_64;
use byteorder::{LittleEndian, ReadBytesExt};
use rand_core::{impls, RngCore, SeedableRng};
use serde_derive::*;

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Hash64Gen {
    pub state: u64,
    pub seed: u64,
}

impl Hash64Gen {
    pub fn jump_forward(&mut self, by: u64) {
        self.state = self.state.wrapping_add(by);
    }

    pub fn jump_backwards(&mut self, by: u64) {
        self.state = self.state.wrapping_sub(by);
    }
}

impl RngCore for Hash64Gen {
    fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }

    fn next_u64(&mut self) -> u64 {
        let v = seed_hash_64(self.state, self.seed);
        self.state = self.state.wrapping_add(1);
        v
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        impls::fill_bytes_via_next(self, dest)
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
        Ok(self.fill_bytes(dest))
    }
}

impl SeedableRng for Hash64Gen {
    type Seed = [u8; core::mem::size_of::<u64>()];

    /// Beware, a seed of 0 results in the first output being 0.
    fn from_seed(seed: Self::Seed) -> Self {
        Self::seed_from_u64((&seed as &[u8]).read_u64::<LittleEndian>().unwrap())
    }

    /// Beware, a seed of 0 results in the first output being 0.
    fn seed_from_u64(seed: u64) -> Self {
        Hash64Gen { state: 0, seed }
    }
}
