// #![allow(unused)]

use color_eyre::{owo_colors::OwoColorize, Result};
use tokio::time::{sleep, Duration};

use secretrs::proto::cosmos::tx::v1beta1::BroadcastTxRequest;
use secretrs::{
    bank::MsgSend,
    clients::{GrpcClient, TxServiceClient},
    query::PageRequest,
    tendermint::Hash,
    tx::{self, Fee, Msg, SignDoc, SignerInfo, Tx},
    AccountId, Coin,
};

// const GRPC_URL: &str = "http://grpc.testnet.secretsaturn.net:9090";
const GRPC_URL: &str = "http://localhost:9090";

async fn async_main() -> Result<()> {
    // A single item page used throughout for brevity
    let _one_page = Some(PageRequest {
        key: vec![],
        offset: 0,
        limit: 1,
        count_total: true,
        reverse: false,
    });

    // Tx Broadcast
    println!("\n{}", "Tx Service".underline().blue());
    println!("Creating `tx` service client...");

    let mut tx_client = TxServiceClient::connect(GRPC_URL).await?;

    let account = secretrs::account::a();

    let sender_private_key = account.signing_key();
    let sender_public_key = sender_private_key.public_key();
    let sender_account_id = sender_public_key.account_id("secret")?;

    let recipient_account_id =
        "secret1fc3fzy78ttp0lwuujw7e52rhspxn8uj52zfyne".parse::<AccountId>()?;

    let amount = Coin {
        amount: 1_000_000u128,
        denom: "uscrt".parse()?,
    };

    let msg_send = MsgSend {
        from_address: sender_account_id.clone(),
        to_address: recipient_account_id,
        amount: vec![amount.clone()],
    };

    let chain_id = "secretdev-1".parse()?;
    let account_number = 0;
    let sequence_number = 3;
    let gas = 100_000u64;
    let timeout_height = 99999999u32;
    let memo = "example memo";

    let tx_body = tx::Body::new(vec![msg_send.to_any()?], memo, timeout_height);
    let signer_info = SignerInfo::single_direct(Some(sender_public_key), sequence_number);
    let auth_info = signer_info.auth_info(Fee::from_amount_and_gas(amount, gas));
    let sign_doc = SignDoc::new(&tx_body, &auth_info, &chain_id, account_number)?;

    let tx_signed = sign_doc.sign(&sender_private_key)?;
    let tx_bytes = tx_signed.to_bytes()?;

    let response = tx_client
        .broadcast_tx(BroadcastTxRequest { tx_bytes, mode: 1 })
        .await?
        .into_inner()
        .tx_response
        .unwrap();

    println!("Response => {:?}", response.cyan());
    println!("Gas Used => {:?}", response.gas_used.yellow());
    println!("Gas Wanted => {:?}", response.gas_wanted.yellow());

    let tx_hash = Hash::try_from(hex::decode(response.txhash)?)?;

    sleep(Duration::from_secs(2)).await;

    let tx = Tx::grpc_find_by_hash(&mut tx_client, tx_hash).await?;
    println!("Tx => {:?}", tx.purple());

    Ok(())
}

fn main() -> Result<()> {
    color_eyre::install()?;

    // Create a new Tokio runtime using the current thread scheduler
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_io()
        .enable_time()
        .build()
        .unwrap();

    // Use the runtime to run the async code
    rt.block_on(async_main())
}
