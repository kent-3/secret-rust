#![allow(unused)]

use super::{Error, Result};
use crate::{
    client::{query::Querier, tx::TxSender},
    proto::cosmos::tx::v1beta1::{AuthInfo, BroadcastMode, SignDoc, SignerInfo, Tx, TxBody, TxRaw},
    EncryptionUtils,
};
use std::sync::Arc;
use tonic::codegen::{Body, Bytes, StdError};

#[derive(Debug, Clone)]
pub struct SecretNetworkClient<T>
where
    T: tonic::client::GrpcService<tonic::body::BoxBody>,
    T::Error: Into<StdError>,
    T::ResponseBody: Body<Data = Bytes> + Send + 'static,
    <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    T: Clone,
{
    pub url: &'static str,
    pub query: Querier<T>,
    pub tx: TxSender<T>,
    // TODO - figure out this wallet business
    pub wallet: Option<Wallet>,
    pub address: String,
    pub chain_id: &'static str,
    pub encryption_utils: EncryptionUtils,
    // TODO - is this worth doing?
    // tx_options: Arc<TxOptions>,
}

#[cfg(not(target_arch = "wasm32"))]
impl SecretNetworkClient<::tonic::transport::Channel> {
    pub async fn connect(options: CreateClientOptions) -> Result<Self> {
        let channel = tonic::transport::Channel::from_static(options.url)
            .connect()
            .await?;
        Ok(Self::new(channel, options)?)
    }

    pub fn new(channel: ::tonic::transport::Channel, options: CreateClientOptions) -> Result<Self> {
        let url = options.url;

        let query = Querier::new(channel.clone(), &options);
        let tx = TxSender::new(channel.clone(), &options);
        // let tx_options = Arc::new(TxOptions::default());

        let wallet = options.wallet;
        let address = options.wallet_address.unwrap_or_default();
        let chain_id = options.chain_id;

        let encryption_utils = EncryptionUtils::new(options.encryption_seed, options.chain_id)?;

        Ok(Self {
            url,
            query,
            tx,
            wallet,
            address,
            chain_id,
            encryption_utils, // tx_options,
        })
    }

    // pub fn tx_options(&mut self, options: TxOptions) -> &mut Self {
    //     self.tx_options = Arc::new(options);
    //     self
    // }
}

/// Options to configure the creation of a client.
#[derive(Debug)]
pub struct CreateClientOptions {
    /// A URL to the API service, also known as LCD, REST API or gRPC-gateway, typically on port 1317.
    pub url: &'static str,
    /// The chain-id used in encryption code & when signing transactions.
    pub chain_id: &'static str,
    /// An optional wallet for signing transactions & permits. If `wallet` is supplied,
    /// `wallet_address` & `chain_id` must also be supplied.
    pub wallet: Option<Wallet>,
    /// The specific account address in the wallet that is permitted to sign transactions & permits.
    pub wallet_address: Option<String>,
    /// Optional encryption seed that will allow transaction decryption at a later time.
    /// Ignored if `encryption_utils` is supplied. Must be 32 bytes.
    pub encryption_seed: Option<[u8; 32]>,
    /// Optional field to override the default encryption utilities implementation.
    pub encryption_utils: Option<EncryptionUtils>,
}

impl Default for CreateClientOptions {
    fn default() -> Self {
        Self {
            url: "http://localhost:9090",
            chain_id: "secretdev-1",
            wallet: None,
            wallet_address: None,
            encryption_seed: None,
            encryption_utils: None,
        }
    }
}

impl CreateClientOptions {
    pub fn read_only(url: &'static str, chain_id: &'static str) -> Self {
        Self {
            url,
            chain_id,
            ..Default::default()
        }
    }
}

/// A signer capable of signing transactions.
/// Placeholder for actual implementation details.
#[derive(Debug, Clone)]
pub struct Wallet {
    // Define methods relevant to the Signer trait here
}

/// Options for transactions
#[derive(Debug, Clone)]
pub struct TxOptions {
    /// Gas limit for the transaction, defaults to `50_000`
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
    pub ibc_txs_options: Option<IbcTxOptions>,
}

impl Default for TxOptions {
    fn default() -> Self {
        Self {
            gas_limit: 50_000,
            gas_price_in_fee_denom: 0.1,
            fee_denom: "uscrt".to_string(),
            fee_granter: None,
            memo: String::default(),
            wait_for_commit: true,
            broadcast_timeout_ms: 60_000,
            broadcast_check_interval_ms: 6_000,
            broadcast_mode: BroadcastMode::Sync,
            explicit_signer_data: None,
            ibc_txs_options: Some(IbcTxOptions::default()),
        }
    }
}

/// Signer data for overriding chain-specific data
#[derive(Debug, Clone)]
pub struct SignerData {
    pub account_number: u32,
    pub account_sequence: u32,
    pub chain_id: String,
}

/// Options related to IBC transactions
#[derive(Debug, Clone)]
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
