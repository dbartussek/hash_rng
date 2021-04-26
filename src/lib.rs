#![no_std]

mod gen32;
mod gen64;
pub mod hash;

pub use self::{gen32::*, gen64::*};
