use color_eyre::{eyre::OptionExt, owo_colors::OwoColorize, Result};
use secretrs::{proto, AuthQueryClient, BankQueryClient, ComputeQueryClient};
use tonic::IntoRequest;

const GRPC_URL: &str = "http://grpc.testnet.secretsaturn.net:9090";
const TEST_ADDRESS: &str = "secret1ap26qrlp8mcq2pg6r47w43l0y8zkqm8a450s03";

async fn async_main() -> Result<()> {
    // A single item page used throughout for brevity
    use proto::cosmos::base::query::v1beta1::PageRequest;
    let _one_page = Some(PageRequest {
        key: vec![],
        offset: 0,
        limit: 1,
        count_total: true,
        reverse: false,
    });

    // Auth Queries
    println!("\n{}", "Auth Module".underline().blue());
    println!("Creating `auth` query client...");
    let mut secret_auth = AuthQueryClient::connect(GRPC_URL).await?;

    let request = proto::cosmos::auth::v1beta1::QueryAccountRequest {
        address: TEST_ADDRESS.to_string(),
    };
    println!("Request => {:?}", request.green());

    let response = secret_auth.account(request.into_request()).await?;

    let (metadata, response, _extensions) = response.into_parts();
    println!("Response => {:?}", response.green());

    // Extract the block height of the response from the headers
    let http_headers = metadata.into_headers();
    let block_height = http_headers
        .get("x-cosmos-block-height")
        .ok_or_eyre("Missing header")?;

    // Method #1
    if let Some(any) = response.account {
        let base_account = any.to_msg::<proto::cosmos::auth::v1beta1::BaseAccount>()?;
        let base_account = secretrs::auth::BaseAccount::try_from(base_account)?;
        println!(
            "Example: \"{}'s account number is {} and sequence is {} at block {}\"",
            base_account.address.bright_green(),
            base_account.account_number.bright_yellow(),
            base_account.sequence.bright_yellow(),
            block_height.to_str()?.bright_yellow()
        );
    }

    // Method #2
    // let base_account = response
    //     .account
    //     .and_then(|any| any.to_msg::<proto::cosmos::auth::v1beta1::BaseAccount>().ok() )
    //     .and_then(|base_account| secretrs::auth::BaseAccount::try_from(base_account).ok())
    //     .ok_or_eyre("No Account")?;
    // println!("Account: {:?}", base_account.green());

    // Bank Queries
    println!("\n{}", "Bank Module".underline().blue());
    println!("Creating `bank` query client...");
    let mut secret_bank = BankQueryClient::connect(GRPC_URL).await?;

    let request = proto::cosmos::bank::v1beta1::QueryBalanceRequest {
        address: TEST_ADDRESS.to_string(),
        denom: "uscrt".to_string(),
    };
    println!("Request => {:?}", request.green());

    let response = secret_bank.balance(request.into_request()).await?;

    let (metadata, response, _extensions) = response.into_parts();

    println!("Response => {:?}", response.green());

    let http_headers = metadata.into_headers();
    let block_height = http_headers
        .get("x-cosmos-block-height")
        .ok_or_eyre("Missing header")?;

    let balance = response.balance.unwrap().amount;
    println!(
        "Example: \"{} has a balance of {} uscrt at block {}\"",
        TEST_ADDRESS.bright_green(),
        balance.bright_yellow(),
        block_height.to_str()?.bright_yellow()
    );

    // Compute Queries
    println!("\n{}", "Compute Module".underline().blue());
    println!("Creating `compute` query client...");
    let mut secret_compute = ComputeQueryClient::connect(GRPC_URL).await?;

    let request = proto::secret::compute::v1beta1::QueryByCodeIdRequest { code_id: 1 };
    println!("Request => {:?}", request.green());

    let response = secret_compute
        .code_hash_by_code_id(request.into_request())
        .await?;

    let response = response.into_inner();
    println!("Response => {:?}", response.green());

    Ok(())
}

fn main() -> Result<()> {
    color_eyre::install()?;

    // Create a new Tokio runtime using the current thread scheduler
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_io()
        .build()
        .unwrap();

    // Use the runtime to run the async code
    rt.block_on(async_main())
}
