use crate::{tx::Msg, AccountId, ErrorReport, Result};
use secret_sdk_proto as proto;

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

/// MsgStoreCodeResponse returns store result data.
#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct MsgStoreCodeResponse {
    /// CodeID is the reference to the stored WASM code
    pub code_id: u64,
    // Secret doesn't return a checksum in the MsgStoreCodeResponse
    // /// Checksum is the sha256 hash of the stored code
    // pub checksum: Hash,
}

impl Msg for MsgStoreCodeResponse {
    type Proto = proto::secret::compute::v1beta1::MsgStoreCodeResponse;
}

impl TryFrom<proto::secret::compute::v1beta1::MsgStoreCodeResponse> for MsgStoreCodeResponse {
    type Error = ErrorReport;

    fn try_from(
        proto: proto::secret::compute::v1beta1::MsgStoreCodeResponse,
    ) -> Result<MsgStoreCodeResponse> {
        Ok(MsgStoreCodeResponse {
            code_id: proto.code_id,
            // checksum: Hash::Sha256(proto.checksum.try_into().map_err(|_| Error::Crypto)?),
        })
    }
}

impl From<MsgStoreCodeResponse> for proto::secret::compute::v1beta1::MsgStoreCodeResponse {
    fn from(msg: MsgStoreCodeResponse) -> proto::secret::compute::v1beta1::MsgStoreCodeResponse {
        proto::secret::compute::v1beta1::MsgStoreCodeResponse {
            code_id: msg.code_id,
            // checksum: msg.checksum.as_bytes().into(),
        }
    }
}
