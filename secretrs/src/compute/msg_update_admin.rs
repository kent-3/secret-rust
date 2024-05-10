use crate::{tx::Msg, AccountId, ErrorReport, Result};
use secret_sdk_proto as proto;

/// MsgUpdateAdmin sets a new admin for a smart contract
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct MsgUpdateAdmin {
    /// Sender is the that actor that signed the messages
    pub sender: AccountId,

    /// NewAdmin address to be set
    pub new_admin: AccountId,

    /// Contract is the address of the smart contract
    pub contract: AccountId,
}

impl Msg for MsgUpdateAdmin {
    type Proto = proto::secret::compute::v1beta1::MsgUpdateAdmin;
}

impl TryFrom<proto::secret::compute::v1beta1::MsgUpdateAdmin> for MsgUpdateAdmin {
    type Error = ErrorReport;

    fn try_from(proto: proto::secret::compute::v1beta1::MsgUpdateAdmin) -> Result<MsgUpdateAdmin> {
        MsgUpdateAdmin::try_from(&proto)
    }
}

impl TryFrom<&proto::secret::compute::v1beta1::MsgUpdateAdmin> for MsgUpdateAdmin {
    type Error = ErrorReport;

    fn try_from(proto: &proto::secret::compute::v1beta1::MsgUpdateAdmin) -> Result<MsgUpdateAdmin> {
        Ok(MsgUpdateAdmin {
            sender: proto.sender.parse()?,
            new_admin: proto.new_admin.parse()?,
            contract: proto.contract.parse()?,
        })
    }
}

impl From<MsgUpdateAdmin> for proto::secret::compute::v1beta1::MsgUpdateAdmin {
    fn from(msg: MsgUpdateAdmin) -> proto::secret::compute::v1beta1::MsgUpdateAdmin {
        proto::secret::compute::v1beta1::MsgUpdateAdmin::from(&msg)
    }
}

impl From<&MsgUpdateAdmin> for proto::secret::compute::v1beta1::MsgUpdateAdmin {
    fn from(msg: &MsgUpdateAdmin) -> proto::secret::compute::v1beta1::MsgUpdateAdmin {
        proto::secret::compute::v1beta1::MsgUpdateAdmin {
            sender: msg.sender.to_string(),
            new_admin: msg.new_admin.to_string(),
            contract: msg.contract.to_string(),
            callback_sig: vec![],
        }
    }
}

/// MsgUpdateAdminResponse returns empty data
#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct MsgUpdateAdminResponse {}

impl Msg for MsgUpdateAdminResponse {
    type Proto = proto::secret::compute::v1beta1::MsgUpdateAdminResponse;
}

impl TryFrom<proto::secret::compute::v1beta1::MsgUpdateAdminResponse> for MsgUpdateAdminResponse {
    type Error = ErrorReport;

    fn try_from(
        _proto: proto::secret::compute::v1beta1::MsgUpdateAdminResponse,
    ) -> Result<MsgUpdateAdminResponse> {
        Ok(MsgUpdateAdminResponse {})
    }
}

impl From<MsgUpdateAdminResponse> for proto::secret::compute::v1beta1::MsgUpdateAdminResponse {
    fn from(
        _msg: MsgUpdateAdminResponse,
    ) -> proto::secret::compute::v1beta1::MsgUpdateAdminResponse {
        proto::secret::compute::v1beta1::MsgUpdateAdminResponse {}
    }
}
