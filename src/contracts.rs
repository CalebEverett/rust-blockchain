use std::env;
use web3::{
    contract::Contract,
    types::{Address, BlockNumber, Filter, FilterBuilder, Log},
};

pub async fn get_contract() -> Result<(), Box<dyn std::error::Error>> {
    let url = format!(
        "https://mainnet.infura.io/v3/{}",
        env::var("WEB3_INFURA_PROJECT_ID")?
    );
    let http = web3::transports::Http::new(&url)?;
    let web3 = web3::Web3::new(http);

    let address: Address = "0x059EDD72Cd353dF5106D2B9cC5ab83a52287aC3a"
        .parse()
        .unwrap();
    println!("{:?}", &address);
    let contract = Contract::from_json(
        web3.eth(),
        address,
        include_bytes!(
            "../src/events/contracts_json/0x059EDD72Cd353dF5106D2B9cC5ab83a52287aC3a.json"
        ),
    )?;

    let topic0 = contract.abi().event("Transfer")?.signature();
    let filter: Filter = FilterBuilder::default()
        .from_block(BlockNumber::from(11341538))
        .to_block(BlockNumber::from(11341548))
        .address(vec![address])
        .topics(Some(vec![topic0]), None, None, None)
        .build();
    let logs: Vec<Log> = web3.eth().logs(filter).await?;
    println!("{:?}", logs[1].topics[3].to_low_u64_be());
    Ok(())
}
