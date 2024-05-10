use crate::{ErrorReport, Result};
use secret_sdk_proto as proto;

/// AbsoluteTxPosition is a unique transaction position that allows for global
/// ordering of transactions.
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct AbsoluteTxPosition {
    /// BlockHeight is the block the contract was created at
    pub block_height: u64,
    /// TxIndex is a monotonic counter within the block (actual transaction index, or gas consumed)
    pub tx_index: u64,
}

impl TryFrom<proto::secret::compute::v1beta1::AbsoluteTxPosition> for AbsoluteTxPosition {
    type Error = ErrorReport;

    fn try_from(
        proto: proto::secret::compute::v1beta1::AbsoluteTxPosition,
    ) -> Result<AbsoluteTxPosition> {
        Ok(AbsoluteTxPosition {
            block_height: proto.block_height as u64,
            tx_index: proto.tx_index,
        })
    }
}

impl From<AbsoluteTxPosition> for proto::secret::compute::v1beta1::AbsoluteTxPosition {
    fn from(pos: AbsoluteTxPosition) -> Self {
        proto::secret::compute::v1beta1::AbsoluteTxPosition {
            block_height: pos.block_height as i64,
            tx_index: pos.tx_index,
        }
    }
}
