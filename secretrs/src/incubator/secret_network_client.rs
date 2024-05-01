#![allow(unused)]

use cosmrs::{AccountId, Coin};
use secret_sdk_proto::cosmos::tx::v1beta1::{BroadcastTxRequest, BroadcastTxResponse};
use tonic::client::GrpcService;

use crate::clients::*;

const GRPC_URL: &str = "http://localhost:9090";

#[derive(Debug)]
pub struct SecretNetworkClient<T> {
    pub query: SecretNetworkQueryClient<T>,
    pub tx: SecretNetworkTxClient<T>,
}

#[derive(Debug)]
pub struct SecretNetworkTxClient<T> {
    pub bank: BankMsgClient<T>,
    pub compute: ComputeMsgClient<T>,
    tx: TxServiceClient<T>,
}

#[derive(Debug)]
pub struct SecretNetworkQueryClient<T> {
    pub auth: AuthQueryClient<T>,
    pub bank: BankQueryClient<T>,
    pub compute: ComputeQueryClient<T>,
}

#[derive(Debug)]
pub struct BankMsgClient<T> {
    inner: TxServiceClient<T>,
}

impl BankMsgClient<::tonic::transport::Channel> {
    pub async fn new() -> Self {
        let tx_client = TxServiceClient::connect(GRPC_URL).await.unwrap();
        Self { inner: tx_client }
    }
}

impl BankMsgClient<::tonic_web_wasm_client::Client> {
    pub fn new() -> Self {
        let web_wasm_client = ::tonic_web_wasm_client::Client::new(GRPC_URL.to_string());
        let tx_client = TxServiceClient::new(web_wasm_client);
        Self { inner: tx_client }
    }
}

impl<T> BankMsgClient<T>
where
    T:,
{
    // I suppose I can take as input the MsgSend object and a TxOptions object
    pub fn send(&mut self, from_address: AccountId, to_address: AccountId, amount: Vec<Coin>) {
        use ::cosmrs::bank::MsgSend;

        let msg = MsgSend {
            from_address,
            to_address,
            amount,
        };

        let request = BroadcastTxRequest {
            tx_bytes: vec![],
            mode: 2,
        };

        todo!()

        // self.perform(request);
    }
}

/*
*
* I would need to add some kind of trait bound like this one from `tendermint-rs::rpc::client`:
*
* /// Perform a request against the RPC endpoint.
* ///
* /// This method is used by the default implementations of specific endpoint methods.
* async fn perform<R>(&self, request: R) -> Result<R::Output, Error>
* where
*     R: SimpleRequest;
*/

#[derive(Debug)]
pub struct ComputeMsgClient<T> {
    inner: TxServiceClient<T>,
}

/// Options for transactions
#[derive(Debug)]
pub struct TxOptions {
    /// Gas limit for the transaction, defaults to `25_000`
    pub gas_limit: u32,
    /// Gas price in fee denomination, defaults to `0.1`
    pub gas_price_in_fee_denom: f32,
    /// Denomination for the fee, defaults to `"uscrt"`
    pub fee_denom: String,
    /// Address of the fee granter
    pub fee_granter: Option<String>,
    /// Memo field of the transaction, defaults to an empty string `""`
    pub memo: String,
    /// Whether to wait for the transaction to commit, defaults to `true`
    pub wait_for_commit: bool,
    /// Timeout for waiting for the transaction to commit, defaults to `60_000` ms
    pub broadcast_timeout_ms: u32,
    /// Interval for checking the transaction commit status, defaults to `6_000` ms
    pub broadcast_check_interval_ms: u32,
    /// Broadcast mode, either synchronous or asynchronous
    pub broadcast_mode: BroadcastMode,
    /// Optional explicit signer data
    pub explicit_signer_data: Option<SignerData>,
    /// Options for resolving IBC ack/timeout transactions
    pub ibc_txs_options: IbcTxOptions,
}

impl Default for TxOptions {
    fn default() -> Self {
        Self {
            gas_limit: 25_000,
            gas_price_in_fee_denom: 0.1,
            fee_denom: "uscrt".to_string(),
            fee_granter: None,
            memo: String::default(),
            wait_for_commit: true,
            broadcast_timeout_ms: 60_000,
            broadcast_check_interval_ms: 6_000,
            broadcast_mode: BroadcastMode::Sync,
            explicit_signer_data: None,
            ibc_txs_options: IbcTxOptions::default(),
        }
    }
}

/// Modes of broadcasting transactions
#[derive(Debug)]
pub enum BroadcastMode {
    Block,
    Sync,
    Async,
}

/// Signer data for overriding chain-specific data
#[derive(Debug)]
pub struct SignerData {
    pub account_number: u32,
    pub account_sequence: u32,
    pub chain_id: String,
}

/// Options related to IBC transactions
#[derive(Debug)]
pub struct IbcTxOptions {
    /// If `false`, skip resolving the IBC response txs (acknowledge/timeout).
    ///
    /// Defaults to `true` when broadcasting a tx or using `getTx()`.
    /// Defaults to `false` when using `txsQuery()`.
    resolve_responses: bool,
    /// How much time (in milliseconds) to wait for IBC response txs (acknowledge/timeout).
    ///
    /// Defaults to `120_000` (2 minutes).
    resolve_responses_timeout_ms: u32,
    /// When waiting for the IBC response txs (acknowledge/timeout) to commit on-chain, how much time (in milliseconds) to wait between checks.
    ///
    /// Smaller intervals will cause more load on your node provider. Keep in mind that blocks on Secret Network take about 6 seconds to finalize.
    ///
    /// Defaults to `15_000` (15 seconds).
    resolve_responses_check_interval_ms: u32,
}

impl Default for IbcTxOptions {
    fn default() -> Self {
        Self {
            resolve_responses: true,
            resolve_responses_timeout_ms: 120_000,
            resolve_responses_check_interval_ms: 15_000,
        }
    }
}

/// SignDoc is the type used for generating sign bytes for SIGN_MODE_DIRECT.
#[derive(Debug, Clone)]
#[allow(non_snake_case)]
pub struct SignDocCamelCase {
    /// `bodyBytes` is protobuf serialization of a TxBody that matches the
    /// representation in TxRaw.
    pub bodyBytes: Vec<u8>,

    /// `authInfoBytes` is a protobuf serialization of an AuthInfo that matches the
    /// representation in TxRaw.
    pub authInfoBytes: Vec<u8>,

    /// `chainId` is the unique identifier of the chain this transaction targets.
    /// It prevents signed transactions from being used on another chain by an
    /// attacker.
    pub chainId: String,

    /// `accountNumber` is the account number of the account in state.
    pub accountNumber: String,
}
