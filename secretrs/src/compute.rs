//! Compute messages

mod msg_execute_contract;
mod msg_initiate_contract;
mod msg_store_code;

pub use self::{
    msg_execute_contract::MsgExecuteContract, msg_initiate_contract::MsgInstantiateContract,
    msg_store_code::MsgStoreCode,
};
