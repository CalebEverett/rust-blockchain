use std::env;
use std::path::PathBuf;

mod contracts_schema;

#[derive(Debug)]
struct Contract {
    address: String,
    first_block: u32,
    file_path: PathBuf,
}

impl Contract {
    fn new(address: String, first_block: u32) -> Self {
        let result: String = address.chars().filter(|c| c.is_alphabetic()).collect();
        let file_path = PathBuf::from(format!("./contracts_json/{}.json", result));
        Contract {
            address,
            first_block,
            file_path,
        }
    }
}

pub async fn write_csv(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let infura_key = "WEB3_INFURA_PROJECT_ID";
    println!("{}", env::var(&infura_key)?);
    // get_events(
    //     "0x059EDD72Cd353dF5106D2B9cC5ab83a52287aC3a",
    //     &11341538,
    //     &11351538,
    //     "0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef",
    // )
    // .await?;

    get_events_web3().await?;
    let contract = Contract::new(
        String::from("0x059EDD72Cd353dF5106D2B9cC5ab83a52287aC3a"),
        11341538,
    );
    println!("{:?}", contract);
    Ok(())
}

async fn get_events(
    address: &str,
    from_block: &u32,
    to_block: &u32,
    topic_0_text: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // let topic_0_hash = web3::signing::keccak256(topic_0_text.as_bytes());
    // let topic_0_text = hex::encode(topic_0_hash);
    // let mut params = HashMap::new();
    // let method_params = format!(
    //     "[{{address: {}, fromBlock: {:x}, toBlock: {:x}, topics: [{}]}}]",
    //     address, from_block, to_block, topic_0_text
    // );
    #[derive(serde::Serialize, serde::Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    struct MethodParams {
        address: String,
        from_block: String,
        to_block: String,
        topics: Vec<String>,
    }
    #[derive(serde::Serialize, serde::Deserialize, Debug)]
    struct Params {
        jsonrpc: String,
        method: String,
        id: String,
        params: Vec<MethodParams>,
    }
    let params = Params {
        jsonrpc: "2.0".to_string(),
        method: "eth_getLogs".to_string(),
        id: "1".to_string(),
        params: vec![MethodParams {
            address: "0x059EDD72Cd353dF5106D2B9cC5ab83a52287aC3a".to_string(),
            from_block: format!("0x{:x}", &11341538),
            to_block: format!("0x{:x}", &11341539),
            topics: vec![
                "0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef".to_string(),
            ],
        }],
    };

    #[derive(serde::Serialize, serde::Deserialize, Debug)]
    struct RpcResult {
        jsonrpc: String,
        id: String,
        result: Vec<EthLog>,
    }

    #[derive(serde::Serialize, serde::Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    struct EthLog {
        address: String,
        block_hash: String,
        block_number: String,
        data: String,
        log_index: String,
        removed: bool,
        topics: Vec<String>,
    }
    #[derive(serde::Serialize, serde::Deserialize, Debug)]
    enum RpcRecord {
        EthLog(EthLog),
    }

    let client = reqwest::Client::new();
    let url = format!(
        "https://mainnet.infura.io/v3/{}",
        env::var("WEB3_INFURA_PROJECT_ID")?
    );
    let resp = client
        .post(url)
        .json(&params)
        .send()
        .await?
        .json::<RpcResult>()
        .await?;
    println!("{:?}", &resp);
    let records = resp.result;
    let mut wtr = csv::Writer::from_path("data/events.csv")?;
    for r in &records {
        wtr.serialize(r)?;
    }
    wtr.flush()?;
    Ok(())
}
