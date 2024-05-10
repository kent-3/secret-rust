//! Not implemented
//!
//! `secret::compute::v1beta1::AccessConfig` does not exist

// use super::AccessType;
// use crate::{AccountId, Error, ErrorReport, Result};
// use secret_sdk_proto as proto;
//
// /// AccessConfig access control type.
// #[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
// pub struct AccessConfig {
//     /// Access type granted.
//     pub permission: AccessType,
//
//     /// Account addresses with the associated permission.
//     pub addresses: Vec<AccountId>,
// }
//
// impl TryFrom<proto::secret::compute::v1beta1::AccessConfig> for AccessConfig {
//     type Error = ErrorReport;
//
//     fn try_from(proto: proto::secret::compute::v1beta1::AccessConfig) -> Result<AccessConfig> {
//         AccessConfig::try_from(&proto)
//     }
// }
//
// impl TryFrom<&proto::secret::compute::v1beta1::AccessConfig> for AccessConfig {
//     type Error = ErrorReport;
//
//     fn try_from(proto: &proto::secret::compute::v1beta1::AccessConfig) -> Result<AccessConfig> {
//         let permission =
//             AccessType::try_from(proto.permission).map_err(|_| Error::InvalidEnumValue {
//                 name: "permission",
//                 found_value: proto.permission,
//             })?;
//
//         let mut addresses = Vec::with_capacity(proto.addresses.len());
//
//         if !proto.address.is_empty() {
//             addresses.push(proto.address.parse()?);
//         }
//
//         for address in &proto.addresses {
//             addresses.push(address.parse()?);
//         }
//
//         Ok(AccessConfig {
//             permission,
//             addresses,
//         })
//     }
// }
//
// impl From<AccessConfig> for proto::secret::compute::v1beta1::AccessConfig {
//     fn from(config: AccessConfig) -> proto::secret::compute::v1beta1::AccessConfig {
//         proto::secret::compute::v1beta1::AccessConfig::from(&config)
//     }
// }
//
// impl From<&AccessConfig> for proto::secret::compute::v1beta1::AccessConfig {
//     fn from(config: &AccessConfig) -> proto::secret::compute::v1beta1::AccessConfig {
//         proto::secret::compute::v1beta1::AccessConfig {
//             permission: config.permission as i32,
//             address: "".to_string(),
//             addresses: config.addresses.iter().map(ToString::to_string).collect(),
//         }
//     }
// }
