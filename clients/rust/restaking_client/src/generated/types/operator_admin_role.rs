//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! <https://github.com/kinobi-so/kinobi>
//!

use borsh::BorshDeserialize;
use borsh::BorshSerialize;
use num_derive::FromPrimitive;

#[derive(
    BorshSerialize,
    BorshDeserialize,
    Clone,
    Debug,
    Eq,
    PartialEq,
    Copy,
    PartialOrd,
    Hash,
    FromPrimitive,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum OperatorAdminRole {
    NcnAdmin,
    VaultAdmin,
    VoterAdmin,
    DelegateAdmin,
    MetadataAdmin,
}
