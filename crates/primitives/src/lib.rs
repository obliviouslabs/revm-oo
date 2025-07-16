//! # revm-primitives
//!
//! EVM primitive types.
#![cfg_attr(not(test), warn(unused_crate_dependencies))]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc as std;

pub mod constants;
pub mod eip170;
pub mod eip3860;
pub mod eip4844;
pub mod eip7702;
pub mod eip7823;
pub mod eip7825;
pub mod eip7907;
pub mod eip7918;
pub mod hardfork;

pub use constants::*;

// Reexport alloy primitives.

pub use alloy_primitives::map::{self, hash_map, hash_set, HashMap, HashSet};
pub use alloy_primitives::{
    self, address, b256, bytes, fixed_bytes, hex, hex_literal, keccak256, ruint, uint, Address,
    Bytes, FixedBytes, Log, LogData, TxKind, B256, I128, I256, U128, U256,
};

/// type alias for storage keys
pub type StorageKey = U256;
/// type alias for storage values
pub type StorageValue = U256;

use bytemuck::{Pod, Zeroable};
use rostl_primitives::traits::{Cmov, _Cmovbase};

#[repr(transparent)]
#[derive(Copy, Clone, Default)]
pub struct OU256(pub U256);

unsafe impl Zeroable for OU256 {}
unsafe impl Pod for OU256 {}

use rostl_primitives::{impl_cmov_for_pod, cmov_body, cxchg_body};
impl_cmov_for_pod!(OU256);

impl From<U256> for OU256 {
    fn from(x: U256) -> Self {
        OU256(x)
    }
}

impl From<OU256> for U256 {
    fn from(x: OU256) -> Self {
        x.0
    }
}

// Define a constant for zero and one:
pub const OU256_ZERO: OU256 = OU256(U256::ZERO);
pub const OU256_ONE: OU256 = OU256(U256::ONE);
