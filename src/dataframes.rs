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
