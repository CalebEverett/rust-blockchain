use polars::prelude::{CsvReader, DataFrame, DataType, Field, Schema, SerReader};

pub async fn get_projects_df(path: &str) -> Result<DataFrame, Box<dyn std::error::Error>> {
    let schema = Schema::new(vec![
        Field::new("id", DataType::Utf8),
        Field::new("projectId", DataType::UInt32),
        Field::new("name", DataType::Utf8),
        Field::new("artistName", DataType::Utf8),
        Field::new("curationStatus", DataType::Utf8),
        Field::new("invocations", DataType::UInt32),
        Field::new("maxInvocations", DataType::UInt32),
        Field::new("dynamic", DataType::Boolean),
        Field::new("scriptJSON", DataType::Utf8),
        Field::new("website", DataType::Utf8),
        Field::new("license", DataType::Utf8),
        Field::new("active", DataType::Boolean),
        Field::new("paused", DataType::Boolean),
    ]);
    let df = CsvReader::from_path(&path)?
        .with_schema(&schema)
        .has_header(true)
        .finish()
        .expect("Failed to create dataframe from csv");

    Ok(df)
}

// address,event,topic1,topic2,topic3,data,block_hash,block_number,transaction_hash,transaction_index,log_index,transaction_log_index,log_type,removed

pub async fn get_events_df(
    address: &str,
    event: &str,
) -> Result<DataFrame, Box<dyn std::error::Error>> {
    let schema = Schema::new(vec![
        Field::new("address", DataType::Utf8),
        Field::new("event", DataType::Utf8),
        Field::new("topic1", DataType::Utf8),
        Field::new("topic2", DataType::Utf8),
        Field::new("topic3", DataType::Utf8),
        Field::new("data", DataType::Utf8),
        Field::new("blockHash", DataType::Utf8),
        Field::new("blockNumber", DataType::Utf8),
        Field::new("transactionHash", DataType::Utf8),
        Field::new("logIndex", DataType::Utf8),
        Field::new("transactionIndex", DataType::Utf8),
        Field::new("transactionLogIndex", DataType::Utf8),
        Field::new("logType", DataType::Utf8),
        Field::new("removed", DataType::Boolean),
    ]);
    let path = format!("./data/{}/{}/records.csv", address, event);
    let df = CsvReader::from_path(&path)?
        .with_schema(&schema)
        .has_header(true)
        .finish()
        .expect("Failed to create dataframe from csv");

    Ok(df)
}
