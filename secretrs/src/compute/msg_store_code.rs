use crate::{tx::Msg, AccountId, ErrorReport, Result};
use secret_sdk_proto as proto;
use std::convert::TryFrom;

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
/// MsgStoreCode submit Wasm code to the system
pub struct MsgStoreCode {
    /// Sender is the that actor that signed the messages
    pub sender: AccountId,
    /// WASMByteCode can be raw or gzip compressed
    pub wasm_byte_code: Vec<u8>,
    /// Source is a valid absolute HTTPS URI to the contract's source code, optional
    pub source: Option<String>,
    /// Builder is a valid docker image name with tag, optional
    pub builder: Option<String>,
}

impl Msg for MsgStoreCode {
    type Proto = proto::secret::compute::v1beta1::MsgStoreCode;
}

impl TryFrom<proto::secret::compute::v1beta1::MsgStoreCode> for MsgStoreCode {
    type Error = ErrorReport;

    fn try_from(proto: proto::secret::compute::v1beta1::MsgStoreCode) -> Result<MsgStoreCode> {
        Ok(MsgStoreCode {
            sender: AccountId::new("secret", &proto.sender)?,
            wasm_byte_code: proto.wasm_byte_code,
            source: if proto.source.is_empty() {
                None
            } else {
                Some(proto.source)
            },
            builder: if proto.builder.is_empty() {
                None
            } else {
                Some(proto.builder)
            },
        })
    }
}

impl From<MsgStoreCode> for proto::secret::compute::v1beta1::MsgStoreCode {
    fn from(msg: MsgStoreCode) -> proto::secret::compute::v1beta1::MsgStoreCode {
        proto::secret::compute::v1beta1::MsgStoreCode {
            sender: msg.sender.to_bytes(),
            wasm_byte_code: msg.wasm_byte_code,
            source: msg.source.unwrap_or_default(),
            builder: msg.builder.unwrap_or_default(),
        }
    }
}
