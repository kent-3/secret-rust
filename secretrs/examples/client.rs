#![allow(unused)]

use cosmrs::bank::MsgSend;
use secretrs::clients::TxServiceClient;
use secretrs::incubator::{CreateClientOptions, Result, SecretNetworkClient, TxOptions};
use secretrs::proto::cosmos::auth::v1beta1::QueryParamsRequest;

const GRPC_URL: &str = "http://grpc.testnet.secretsaturn.net:9090";
const CHAIN_ID: &str = "pulsar-3";

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let opt = CreateClientOptions {
        url: GRPC_URL,
        chain_id: CHAIN_ID,
        wallet: None,
        wallet_address: None,
        encryption_seed: None,
        encryption_utils: None,
    };
    let mut secretrs = SecretNetworkClient::new(opt).await?;
    println!("{:#?}", secretrs);

    let foo = secretrs.query.auth.params().await?;
    println!("{:?}", foo);

    let msg = MsgSend {
        from_address: "foo".parse()?,
        to_address: "bar".parse()?,
        amount: vec![],
    };
    let foo = secretrs.tx.broadcast(vec![msg]);

    // let mut secretrs_tx = TxSender::new(GRPC_URL).await?;
    // println!("{:#?}", secretrs_tx);

    // let msg = MsgSend {
    //     from_address: "foo".parse()?,
    //     to_address: "bar".parse()?,
    //     amount: vec![],
    // };
    // let foo = secretrs_tx.broadcast(vec![msg]);

    // let mut secretrs_query = Querier::new(GRPC_URL).await?;
    // println!("{:#?}", secretrs_query);
    //
    // let foo = secretrs_query.auth.params().await?;
    // println!("{:?}", foo);
    //
    // let bar = secretrs_query
    //     .bank
    //     .params(secretrs::proto::cosmos::bank::v1beta1::QueryParamsRequest {})
    //     .await?;
    // println!("{:?}", bar);

    Ok(())
}
