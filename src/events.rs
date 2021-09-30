//! Functions for interacting with contracts and fetching logs

use log::info;
use std::{
    env,
    fs::{create_dir_all, remove_dir_all, File},
    io::Read,
    path::{Path, PathBuf},
};
use web3::{contract::Contract, transports::Http, types::*, Web3};

/// Returns a web instance for interacting with an infura ethereum node.
pub async fn get_web3() -> Result<Web3<Http>, Box<dyn std::error::Error>> {
    let url = format!(
        "https://mainnet.infura.io/v3/{}",
        env::var("WEB3_INFURA_PROJECT_ID")?
    );
    let http = web3::transports::Http::new(&url)?;
    let web3 = web3::Web3::new(http);
    Ok(web3)
}

/// Returns a contract with transport for interacting with contract on ethereum.
pub async fn get_contract(
    web3: &Web3<Http>,
    address: &str,
) -> Result<Contract<Http>, Box<dyn std::error::Error>> {
    let mut path = PathBuf::from("./data/contracts_json");
    path.push(address);
    path.set_extension("json");
    let mut file = File::open(path)?;
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)?;

    let address: Address = address.parse().unwrap();
    let contract = Contract::from_json(web3.eth(), address, bytes.as_ref())?;
    Ok(contract)
}

/// Returns all events for a contract
pub async fn get_events_all(
    address: &str,
    event: &str,
    from_block: u64,
    to_block: u64,
) -> Result<(), Box<dyn std::error::Error>> {
    let web3 = get_web3().await?;
    let contract = get_contract(&web3, &address).await?;
    let topic0 = contract.abi().event(event)?.signature();
    let address = contract.address();
    let path = format!("./data/events/{}/{}", address, event);
    let path = Path::new(&path);
    if path.exists() {
        remove_dir_all(path)?;
    }
    create_dir_all(path)?;
    get_events(address, topic0, from_block, to_block, event, web3).await?;
    Ok(())
}

pub async fn get_events<'a>(
    address: Address,
    topic0: H256,
    from_block: u64,
    to_block: u64,
    event: &str,
    web3: Web3<Http>,
) -> Result<(), Box<dyn std::error::Error>> {
    #[derive(serde::Serialize)]
    struct LogFlat<'a> {
        pub address: H160,
        pub event: &'a str,
        pub topic1: H256,
        pub topic2: H256,
        pub topic3: H256,
        pub data: Bytes,
        pub block_hash: Option<H256>,
        pub block_number: Option<U64>,
        pub transaction_hash: Option<H256>,
        pub transaction_index: Option<Index>,
        pub log_index: Option<U256>,
        pub transaction_log_index: Option<U256>,
        pub log_type: Option<String>,
        pub removed: Option<bool>,
    }

    fn flatten_log<'a>(log: Log, event: &'a str) -> LogFlat<'a> {
        LogFlat {
            address: log.address,
            event: event,
            topic1: log.topics[1],
            topic2: log.topics[2],
            topic3: log.topics[3],
            data: log.data,
            block_hash: log.block_hash,
            block_number: log.block_number,
            transaction_hash: log.transaction_hash,
            transaction_index: log.transaction_index,
            log_index: log.log_index,
            transaction_log_index: log.transaction_log_index,
            log_type: log.log_type,
            removed: log.removed,
        }
    }
    let filter: Filter = FilterBuilder::default()
        .from_block(BlockNumber::from(from_block))
        .to_block(BlockNumber::from(to_block))
        .address(vec![address])
        .topics(Some(vec![topic0]), None, None, None)
        .build();
    let logs: Vec<Log> = web3.eth().logs(filter).await?;
    let logs_flat: Vec<LogFlat> = logs
        .into_iter()
        .map(|log| flatten_log(log, event))
        .collect();
    if logs_flat.is_empty() {
        info!(
            "No {} events for contract at address {} from block {} to {}.",
            event, address, from_block, to_block
        );
    } else {
        let path = format!(
            "./data/events/{}/{}/{}_{}_{}.csv",
            address,
            event,
            from_block,
            to_block,
            logs_flat.len()
        );
        let mut wtr = csv::Writer::from_path(&path)?;
        for r in &logs_flat {
            wtr.serialize(r)?;
        }
        wtr.flush()?;
        info!("{}", path);
    }
    Ok(())
}
