use super::{AbsoluteTxPosition, ContractCodeId};
use crate::{Error, ErrorReport, Result};
use secret_sdk_proto::{self as proto, secret::compute::v1beta1::ContractCodeHistoryOperationType};

/// ContractCodeHistoryEntry metadata to a contract.
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct ContractCodeHistoryEntry {
    /// The source of this history entry.
    pub operation: ContractCodeHistoryOperationType,

    /// Reference to the stored Wasm code.
    pub code_id: ContractCodeId,

    /// Updated Tx position when the operation was executed.
    pub updated: Option<AbsoluteTxPosition>,

    /// Raw message returned by a wasm contract.
    pub msg: Vec<u8>,
}

impl TryFrom<proto::secret::compute::v1beta1::ContractCodeHistoryEntry>
    for ContractCodeHistoryEntry
{
    type Error = ErrorReport;

    fn try_from(
        proto: proto::secret::compute::v1beta1::ContractCodeHistoryEntry,
    ) -> Result<ContractCodeHistoryEntry> {
        Ok(ContractCodeHistoryEntry {
            operation: ContractCodeHistoryOperationType::try_from(proto.operation).map_err(
                |_| Error::InvalidEnumValue {
                    name: "operation",
                    found_value: proto.operation,
                },
            )?,
            code_id: proto.code_id,
            updated: proto.updated.map(TryFrom::try_from).transpose()?,
            msg: proto.msg,
        })
    }
}
