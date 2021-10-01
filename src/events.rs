//! Functions for interacting with contracts and fetching logs.

use futures::future::join_all;
use log::info;
use std::{
    convert::TryInto,
    env,
    fs::{create_dir_all, remove_dir_all, File, OpenOptions},
    io::Read,
    path::{Path, PathBuf},
};
use tokio::task::JoinHandle;
use web3::{contract::Contract, transports::Http, types::*, Web3};

/// Returns a web instance for interacting with an infura ethereum node.
async fn get_web3() -> Result<Web3<Http>, Box<dyn std::error::Error>> {
    let url = format!(
        "https://mainnet.infura.io/v3/{}",
        env::var("WEB3_INFURA_PROJECT_ID")?
    );
    let http = web3::transports::Http::new(&url)?;
    let web3 = web3::Web3::new(http);
    Ok(web3)
}

/// Returns a contract with transport for interacting with contract on ethereum.
async fn get_contract(
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

/// Returns all contract events for a block range.
pub async fn get_events_all<'a>(
    address_str: &str,
    event: String,
    from_block: u64,
    step: u64,
) -> Result<(), Box<dyn std::error::Error + 'a>> {
    let web3 = get_web3().await?;
    let contract = get_contract(&web3, &address_str).await?;
    let topic0 = contract.abi().event(&event)?.signature();
    let address = contract.address();
    let to_block = web3.eth().block_number().await?;
    let mut tasks: Vec<JoinHandle<Result<(), ()>>> = vec![];
    for i in (from_block..=to_block.low_u64()).step_by(step.try_into().unwrap()) {
        let cloned_event = event.clone();
        let web3 = get_web3().await?;
        let path_str = format!("./data/events/{}/{}", &event, &address_str);
        if Path::new(&path_str).exists() {
            remove_dir_all(&path_str)?;
        }
        create_dir_all(&path_str)?;
        tasks.push(tokio::spawn(async move {
            match get_events(&address, topic0, i, i + step, cloned_event, web3, path_str).await {
                Ok(res) => res,
                Err(_) => (),
            };
            Ok(())
        }));
    }
    join_all(tasks).await;
    Ok(())
}

/// Gets event logs of an address for a block range.
async fn get_events<'a>(
    address: &Address,
    topic0: H256,
    from_block: u64,
    to_block: u64,
    event: String,
    web3: Web3<Http>,
    path_str: String,
) -> Result<(), Box<dyn std::error::Error>> {
    #[derive(serde::Serialize)]
    #[serde(rename_all = "camelCase")]
    struct LogFlat<'a> {
        pub address: H160,
        pub event: &'a str,
        pub topic1: H160,
        pub topic2: H160,
        pub topic3: u64,
        pub data: Bytes,
        pub block_number: u64,
        pub transaction_hash: Option<H256>,
        pub transaction_index: u64,
        pub log_index: u64,
        pub transaction_log_index: Option<U256>,
        pub log_type: Option<String>,
        pub removed: Option<bool>,
    }

    fn flatten_log<'a>(log: Log, event: &'a str) -> LogFlat {
        LogFlat {
            address: log.address,
            event: event,
            topic1: H160::from(log.topics[1]),
            topic2: H160::from(log.topics[2]),
            topic3: log.topics[3].to_low_u64_be(),
            data: log.data,
            block_number: log.block_number.unwrap().low_u64(),
            transaction_hash: log.transaction_hash,
            transaction_index: log.transaction_index.unwrap().low_u64(),
            log_index: log.log_index.unwrap().low_u64(),
            transaction_log_index: log.transaction_log_index,
            log_type: log.log_type,
            removed: log.removed,
        }
    }
    let filter: Filter = FilterBuilder::default()
        .from_block(BlockNumber::from(from_block))
        .to_block(BlockNumber::from(to_block))
        .address(vec![*address])
        .topics(Some(vec![topic0]), None, None, None)
        .build();
    let logs: Vec<Log> = web3.eth().logs(filter).await?;
    let logs_flat: Vec<LogFlat> = logs
        .into_iter()
        .map(|log| flatten_log(log, &event))
        .collect();
    if logs_flat.is_empty() {
        info!(
            "No {} events for contract at address {} from block {} to {}.",
            event, address, from_block, to_block
        );
    } else {
        let mut path = PathBuf::from(&path_str);
        path.push(format!(
            "{}_{}_{}.csv",
            from_block,
            to_block,
            logs_flat.len()
        ));
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(&path)
            .unwrap();
        let mut wtr = csv::Writer::from_writer(file);
        for r in &logs_flat {
            wtr.serialize(r)?;
        }
        wtr.flush()?;
        info!("{}", path.to_str().unwrap());
    }
    Ok(())
}
