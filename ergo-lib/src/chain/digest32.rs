//! Digest types used in ergo-lib and binding crates
//!
//! There is quite same implementation in [ergotree-ir]() crate, but the one here one was introduced
//! before the latter one. The idea was to move all th types there, but for the middle decision,
//! in order not to break compatibility with crates, that use this implementation
//! currently, we decided to create a "remote" implementation for [ergotree-ir]() and use serde "remote" attribute.
//! So the "remote" is actually defined here now.

use crate::chain::{Base16DecodedBytes, Base16EncodedBytes};
use ergotree_ir::chain::digest::Digest;
#[cfg(feature = "json")]
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::convert::TryInto;
use std::fmt::Formatter;
use thiserror::Error;

/// Definition for remote Digest type.
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "json",
    serde(
        into = "Base16EncodedBytes",
        try_from = "Base16DecodedBytes",
        remote = "Digest"
    )
)]
#[derive(PartialEq, Eq, Hash, Clone)]
pub struct DigestDef<const N: usize>(pub Box<[u8; N]>);

/// 32 byte Digest type
pub type Digest32Ref = DigestDef<32>;

impl<const N: usize> DigestDef<N> {
    /// Digest size 32 bytes
    pub const SIZE: usize = N;

    /// All zeros
    pub fn zero() -> DigestDef<N> {
        DigestDef(Box::new([0u8; N]))
    }
}

impl<const N: usize> std::fmt::Debug for DigestDef<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        base16::encode_lower(&(*self.0)).fmt(f)
    }
}

impl<const N: usize> From<DigestDef<N>> for Base16EncodedBytes {
    fn from(v: DigestDef<N>) -> Self {
        Base16EncodedBytes::new(v.0.as_ref())
    }
}

impl<const N: usize> From<Digest<N>> for Base16EncodedBytes {
    fn from(v: Digest<N>) -> Self {
        Base16EncodedBytes::new(v.0.as_ref())
    }
}

impl<const N: usize> From<DigestDef<N>> for String {
    fn from(v: DigestDef<N>) -> Self {
        let bytes: Base16EncodedBytes = v.into();
        bytes.into()
    }
}

impl<const N: usize> TryFrom<Base16DecodedBytes> for DigestDef<N> {
    type Error = Digest32Error;
    fn try_from(bytes: Base16DecodedBytes) -> Result<Self, Self::Error> {
        let arr: [u8; N] = bytes.0.as_slice().try_into()?;
        Ok(DigestDef(Box::new(arr)))
    }
}

impl<const N: usize> TryFrom<Base16DecodedBytes> for Digest<N> {
    type Error = Digest32Error;
    fn try_from(bytes: Base16DecodedBytes) -> Result<Self, Self::Error> {
        let arr: [u8; N] = bytes.0.as_slice().try_into()?;
        Ok(Digest(Box::new(arr)))
    }
}

impl<const N: usize> TryFrom<String> for DigestDef<N> {
    type Error = Digest32Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let bytes = Base16DecodedBytes::try_from(value)?;
        DigestDef::<N>::try_from(bytes)
    }
}

/// Invalid byte array size
#[derive(Error, Debug)]
pub enum Digest32Error {
    /// error decoding from Base16
    #[error("error decoding from Base16: {0}")]
    Base16DecodingError(#[from] base16::DecodeError),
    /// Invalid byte array size
    #[error("Invalid byte array size ({0})")]
    InvalidSize(#[from] std::array::TryFromSliceError),
}

#[cfg(test)]
mod tests {
    use super::DigestDef;
    use proptest::prelude::{Arbitrary, BoxedStrategy};
    use proptest::{collection::vec, prelude::*};
    use std::convert::TryInto;

    impl<const N: usize> Arbitrary for DigestDef<N> {
        type Parameters = ();
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
            vec(any::<u8>(), Self::SIZE)
                .prop_map(|v| DigestDef(Box::new(v.try_into().unwrap())))
                .boxed()
        }
    }
}
