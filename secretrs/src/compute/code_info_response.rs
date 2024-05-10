use super::ContractCodeId;
use crate::{AccountId, ErrorReport, Result};
use secret_sdk_proto as proto;

/// CodeInfoResponse contains code meta data from CodeInfo
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct CodeInfoResponse {
    /// CodeId of the stored contract code.
    pub code_id: ContractCodeId,

    /// Bech32 [`AccountId`] of the creator of this smart contract.
    pub creator: AccountId,

    /// sha256 hash of the code stored
    pub code_hash: String,

    /// Tarball of the source code.
    pub source: String,

    /// Docker image used to compile the wasm file.
    pub builder: String,
}

impl TryFrom<proto::secret::compute::v1beta1::CodeInfoResponse> for CodeInfoResponse {
    type Error = ErrorReport;

    fn try_from(
        proto: proto::secret::compute::v1beta1::CodeInfoResponse,
    ) -> Result<CodeInfoResponse> {
        Ok(CodeInfoResponse {
            code_id: proto.code_id,
            creator: proto.creator.parse()?,
            code_hash: proto.code_hash,
            source: proto.source,
            builder: proto.builder,
        })
    }
}

impl From<CodeInfoResponse> for proto::secret::compute::v1beta1::CodeInfoResponse {
    fn from(code_info: CodeInfoResponse) -> Self {
        proto::secret::compute::v1beta1::CodeInfoResponse {
            code_id: code_info.code_id,
            creator: code_info.creator.to_string(),
            code_hash: code_info.code_hash,
            source: code_info.source,
            builder: code_info.builder,
        }
    }
}
