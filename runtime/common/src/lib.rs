#![cfg_attr(not(feature = "std"), no_std)]
pub use node_primitives::*;
use polkadot_sdk::*;
pub mod opaque {
    use super::*;

    pub use polkadot_sdk::sp_runtime::OpaqueExtrinsic as UncheckedExtrinsic;

    /// Opaque block header type.
    pub type Header =
        sp_runtime::generic::Header<BlockNumber, polkadot_sdk::sp_runtime::traits::BlakeTwo256>;
    /// Opaque block type.
    pub type Block = sp_runtime::generic::Block<Header, UncheckedExtrinsic>;
    /// Opaque block identifier type.
    pub type BlockId = sp_runtime::generic::BlockId<Block>;
}
