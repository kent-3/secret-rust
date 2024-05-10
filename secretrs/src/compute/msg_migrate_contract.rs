use crate::{proto, tx::Msg, AccountId, ErrorReport, Result};

/// MsgMigrateContract runs a code upgrade/ downgrade for a smart contract
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct MsgMigrateContract {
    /// Sender is the that actor that signed the messages
    pub sender: AccountId,

    /// Contract is the address of the smart contract
    pub contract: AccountId,

    /// CodeID references the new WASM code
    pub code_id: u64,

    /// Msg json encoded message to be passed to the contract on migration
    pub msg: Vec<u8>,
}

impl Msg for MsgMigrateContract {
    type Proto = proto::secret::compute::v1beta1::MsgMigrateContract;
}

impl TryFrom<proto::secret::compute::v1beta1::MsgMigrateContract> for MsgMigrateContract {
    type Error = ErrorReport;

    fn try_from(
        proto: proto::secret::compute::v1beta1::MsgMigrateContract,
    ) -> Result<MsgMigrateContract> {
        Ok(MsgMigrateContract {
            sender: proto.sender.parse()?,
            contract: proto.contract.parse()?,
            code_id: proto.code_id,
            msg: proto.msg,
        })
    }
}

impl From<MsgMigrateContract> for proto::secret::compute::v1beta1::MsgMigrateContract {
    fn from(msg: MsgMigrateContract) -> proto::secret::compute::v1beta1::MsgMigrateContract {
        proto::secret::compute::v1beta1::MsgMigrateContract {
            sender: msg.sender.to_string(),
            contract: msg.contract.to_string(),
            code_id: msg.code_id,
            msg: msg.msg,
            callback_code_hash: "".to_string(),
            callback_sig: vec![],
        }
    }
}

/// MsgMigrateContractResponse returns contract migration result data.
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct MsgMigrateContractResponse {
    /// Data contains same raw bytes returned as data from the wasm contract.
    /// (May be empty)
    pub data: Vec<u8>,
}

impl Msg for MsgMigrateContractResponse {
    type Proto = proto::secret::compute::v1beta1::MsgMigrateContractResponse;
}

impl TryFrom<proto::secret::compute::v1beta1::MsgMigrateContractResponse>
    for MsgMigrateContractResponse
{
    type Error = ErrorReport;

    fn try_from(
        proto: proto::secret::compute::v1beta1::MsgMigrateContractResponse,
    ) -> Result<MsgMigrateContractResponse> {
        Ok(MsgMigrateContractResponse { data: proto.data })
    }
}

impl From<MsgMigrateContractResponse>
    for proto::secret::compute::v1beta1::MsgMigrateContractResponse
{
    fn from(
        msg: MsgMigrateContractResponse,
    ) -> proto::secret::compute::v1beta1::MsgMigrateContractResponse {
        proto::secret::compute::v1beta1::MsgMigrateContractResponse { data: msg.data }
    }
}
