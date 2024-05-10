use super::{AbsoluteTxPosition, ContractCodeId};
use crate::{
    proto::{self, traits::ParseOptional},
    AccountId, ErrorReport, Result,
};

/// ContractInfo stores a WASM contract instance
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct ContractInfo {
    /// Reference to the stored Wasm code.
    pub code_id: ContractCodeId,

    /// Creator address who initially instantiated the contract.
    pub creator: AccountId,

    /// Admin is an optional address that can execute migrations.
    pub admin: Option<AccountId>,

    /// Label is optional metadata to be stored with a contract instance.
    pub label: String,

    /// Created Tx position when the contract was instantiated.
    // Note that this data should kept internal and not be exposed via query results.
    // Just use for sorting.
    pub created: Option<AbsoluteTxPosition>,

    /// The IBC port ID assigned to this contract by wasmd.
    /// This is set for all IBC contracts (<https://github.com/CosmWasm/wasmd/blob/v0.16.0/x/wasm/keeper/keeper.go#L299-L306>).
    pub ibc_port_id: String,

    /// Proof that enclave executed the instantiate command.
    pub admin_proof: Vec<u8>,
}

impl TryFrom<proto::secret::compute::v1beta1::ContractInfo> for ContractInfo {
    type Error = ErrorReport;

    fn try_from(proto: proto::secret::compute::v1beta1::ContractInfo) -> Result<ContractInfo> {
        Ok(ContractInfo {
            code_id: proto.code_id,
            creator: AccountId::new("secret", &proto.creator)?,
            admin: proto.admin.parse_optional()?,
            label: proto.label,
            created: proto.created.map(TryFrom::try_from).transpose()?,
            ibc_port_id: proto.ibc_port_id,
            admin_proof: proto.admin_proof,
        })
    }
}

impl From<ContractInfo> for proto::secret::compute::v1beta1::ContractInfo {
    fn from(info: ContractInfo) -> Self {
        proto::secret::compute::v1beta1::ContractInfo {
            code_id: info.code_id,
            creator: info.creator.to_bytes(),
            admin: info
                .admin
                .map(|admin| admin.to_string())
                .unwrap_or_default(),
            label: info.label,
            created: info.created.map(Into::into),
            ibc_port_id: info.ibc_port_id,
            admin_proof: info.admin_proof,
        }
    }
}
