use crate::hash::seed_hash_32;
use byteorder::{LittleEndian, ReadBytesExt};
use rand_core::{impls, RngCore, SeedableRng};
use serde_derive::*;

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Hash32Gen {
    pub state: u32,
    pub seed: u32,
}

impl Hash32Gen {
    pub fn jump_forward(&mut self, by: u32) {
        self.state = self.state.wrapping_add(by);
    }

    pub fn jump_backwards(&mut self, by: u32) {
        self.state = self.state.wrapping_sub(by);
    }
}

impl RngCore for Hash32Gen {
    fn next_u32(&mut self) -> u32 {
        let v = seed_hash_32(self.state, self.seed);
        self.state = self.state.wrapping_add(1);
        v
    }

    fn next_u64(&mut self) -> u64 {
        impls::next_u64_via_u32(self)
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        impls::fill_bytes_via_next(self, dest)
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
        Ok(self.fill_bytes(dest))
    }
}

impl SeedableRng for Hash32Gen {
    type Seed = [u8; core::mem::size_of::<u32>()];

    /// Beware, a seed of 0 results in the first output being 0.
    fn from_seed(seed: Self::Seed) -> Self {
        Self::seed_from_u64((&seed as &[u8]).read_u32::<LittleEndian>().unwrap() as u64)
    }

    /// Beware, a seed of 0 results in the first output being 0.
    fn seed_from_u64(seed: u64) -> Self {
        Hash32Gen {
            state: 0,
            seed: seed as u32,
        }
    }
}
