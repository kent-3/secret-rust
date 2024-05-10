use crate::{tx::Msg, AccountId, Coin, ErrorReport, Result};
use secret_sdk_proto as proto;

#[derive(Debug, Clone)]
/// MsgExecuteContract execute a contract handle function
pub struct MsgExecuteContract {
    /// Sender is the that actor that signed the messages
    pub sender: AccountId,

    /// The contract instance to execute the message on
    pub contract: AccountId,

    /// The message to pass to the contract handle method
    pub msg: Vec<u8>,

    /// Native amounts of coins to send with this message
    pub sent_funds: Vec<Coin>,
}

impl Msg for MsgExecuteContract {
    type Proto = proto::secret::compute::v1beta1::MsgExecuteContract;
}

impl TryFrom<proto::secret::compute::v1beta1::MsgExecuteContract> for MsgExecuteContract {
    type Error = ErrorReport;

    fn try_from(
        proto: proto::secret::compute::v1beta1::MsgExecuteContract,
    ) -> Result<MsgExecuteContract> {
        Ok(MsgExecuteContract {
            sender: AccountId::new("secret", &proto.sender)?,
            contract: AccountId::new("secret", &proto.contract)?,
            msg: proto.msg,
            sent_funds: proto
                .sent_funds
                .into_iter()
                .map(|c| c.try_into())
                .collect::<Result<_>>()?,
        })
    }
}

impl From<MsgExecuteContract> for proto::secret::compute::v1beta1::MsgExecuteContract {
    fn from(msg: MsgExecuteContract) -> proto::secret::compute::v1beta1::MsgExecuteContract {
        proto::secret::compute::v1beta1::MsgExecuteContract {
            sender: msg.sender.to_bytes(),
            contract: msg.contract.to_bytes(),
            msg: msg.msg,
            callback_code_hash: "".to_string(),
            sent_funds: msg.sent_funds.into_iter().map(|c| c.into()).collect(),
            callback_sig: vec![],
        }
    }
}

/// MsgExecuteContractResponse returns execution result data.
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct MsgExecuteContractResponse {
    /// Data contains base64-encoded bytes to returned from the contract
    pub data: Vec<u8>,
}

impl Msg for MsgExecuteContractResponse {
    type Proto = proto::secret::compute::v1beta1::MsgExecuteContractResponse;
}

impl TryFrom<proto::secret::compute::v1beta1::MsgExecuteContractResponse>
    for MsgExecuteContractResponse
{
    type Error = ErrorReport;

    fn try_from(
        proto: proto::secret::compute::v1beta1::MsgExecuteContractResponse,
    ) -> Result<MsgExecuteContractResponse> {
        Ok(MsgExecuteContractResponse { data: proto.data })
    }
}

impl From<MsgExecuteContractResponse>
    for proto::secret::compute::v1beta1::MsgExecuteContractResponse
{
    fn from(
        msg: MsgExecuteContractResponse,
    ) -> proto::secret::compute::v1beta1::MsgExecuteContractResponse {
        proto::secret::compute::v1beta1::MsgExecuteContractResponse { data: msg.data }
    }
}
