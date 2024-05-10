use crate::{tx::Msg, AccountId, ErrorReport, Result};
use secret_sdk_proto as proto;
use std::convert::TryFrom;

/// MsgInstantiateContract initialise a contract from some stored code
#[derive(Debug, Clone)]
pub struct MsgInstantiateContract {
    /// Sender is the that actor that signed the messages
    pub sender: AccountId,
    /// Admin is an optional address that can execute migrations
    pub admin: Option<String>,
    /// The code id of the stored contract code
    pub code_id: u64,
    /// The label to give this contract instance
    pub label: String,
    /// The initialisation message to pass to the contract init method
    pub init_msg: Vec<u8>,
}

impl Msg for MsgInstantiateContract {
    type Proto = proto::secret::compute::v1beta1::MsgInstantiateContract;
}

impl TryFrom<proto::secret::compute::v1beta1::MsgInstantiateContract> for MsgInstantiateContract {
    type Error = ErrorReport;

    fn try_from(
        proto: proto::secret::compute::v1beta1::MsgInstantiateContract,
    ) -> Result<MsgInstantiateContract> {
        Ok(MsgInstantiateContract {
            sender: AccountId::new("secret", &proto.sender)?,
            code_id: proto.code_id,
            label: proto.label,
            init_msg: proto.init_msg,
            admin: Some(proto.admin),
        })
    }
}

impl From<MsgInstantiateContract> for proto::secret::compute::v1beta1::MsgInstantiateContract {
    fn from(
        msg: MsgInstantiateContract,
    ) -> proto::secret::compute::v1beta1::MsgInstantiateContract {
        proto::secret::compute::v1beta1::MsgInstantiateContract {
            sender: msg.sender.to_bytes(),
            code_id: msg.code_id,
            label: msg.label,
            init_msg: msg.init_msg,
            init_funds: vec![],
            admin: "".to_string(),
            callback_code_hash: "".to_string(),
            callback_sig: vec![],
        }
    }
}

/// MsgInstantiateContractResponse return instantiation result data
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct MsgInstantiateContractResponse {
    /// Address is the bech32 address of the new contract instance.
    pub address: AccountId,

    /// Data contains base64-encoded bytes to returned from the contract
    pub data: Vec<u8>,
}

impl Msg for MsgInstantiateContractResponse {
    type Proto = proto::secret::compute::v1beta1::MsgInstantiateContractResponse;
}

impl TryFrom<proto::secret::compute::v1beta1::MsgInstantiateContractResponse>
    for MsgInstantiateContractResponse
{
    type Error = ErrorReport;

    fn try_from(
        proto: proto::secret::compute::v1beta1::MsgInstantiateContractResponse,
    ) -> Result<MsgInstantiateContractResponse> {
        Ok(MsgInstantiateContractResponse {
            address: proto.address.parse()?,
            data: proto.data,
        })
    }
}

impl From<MsgInstantiateContractResponse>
    for proto::secret::compute::v1beta1::MsgInstantiateContractResponse
{
    fn from(
        msg: MsgInstantiateContractResponse,
    ) -> proto::secret::compute::v1beta1::MsgInstantiateContractResponse {
        proto::secret::compute::v1beta1::MsgInstantiateContractResponse {
            address: msg.address.to_string(),
            data: msg.data,
        }
    }
}
