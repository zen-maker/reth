//! Block body abstraction.

use crate::{InMemorySize, MaybeArbitrary};
use alloc::fmt;
use alloy_consensus::Transaction;

/// Abstraction for block's body.
#[auto_impl::auto_impl(&, Arc)]
pub trait BlockBody:
    Send
    + Sync
    + Unpin
    + Clone
    + Default
    + fmt::Debug
    + PartialEq
    + Eq
    + serde::Serialize
    + for<'de> serde::Deserialize<'de>
    + alloy_rlp::Encodable
    + alloy_rlp::Decodable
    + InMemorySize
    + MaybeArbitrary
{
    /// Ordered list of signed transactions as committed in block.
    // todo: requires trait for signed transaction
    type Transaction: Transaction;

    /// Returns reference to transactions in block.
    fn transactions(&self) -> &[Self::Transaction];
}
