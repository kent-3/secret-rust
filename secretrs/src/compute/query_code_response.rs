use super::CodeInfoResponse;
use crate::{proto, ErrorReport, Result};

/// QueryCodeResponse is the response type for the Query/Code RPC method.
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct QueryCodeResponse {
    /// If available, the associated code ID metadata.
    pub code_info: Option<CodeInfoResponse>,

    /// The original wasm bytes.
    pub data: Vec<u8>,
}

impl TryFrom<proto::secret::compute::v1beta1::QueryCodeResponse> for QueryCodeResponse {
    type Error = ErrorReport;

    fn try_from(
        proto: proto::secret::compute::v1beta1::QueryCodeResponse,
    ) -> Result<QueryCodeResponse> {
        Ok(QueryCodeResponse {
            code_info: proto.code_info.map(TryFrom::try_from).transpose()?,
            data: proto.wasm,
        })
    }
}

impl From<QueryCodeResponse> for proto::secret::compute::v1beta1::QueryCodeResponse {
    fn from(response: QueryCodeResponse) -> Self {
        proto::secret::compute::v1beta1::QueryCodeResponse {
            code_info: response.code_info.map(Into::into),
            wasm: response.data,
        }
    }
}
