use log::info;
use polars::prelude::{
    CsvReader, DataFrame, DataType, Field, Result as PolarResult, Schema, SerReader,
};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::{
    fs::{self, File},
    io::Result as IoResult,
    path::{Path, PathBuf},
    sync::Arc,
};

pub async fn get_projects_df(path: &str) -> Result<DataFrame, Box<dyn std::error::Error>> {
    let schema = get_schema(Schemas::Project);
    let df = read_csv(PathBuf::from(path), &schema)?;
    Ok(df)
}

fn read_dir<P: AsRef<Path>>(directory: P) -> IoResult<Vec<PathBuf>> {
    fs::read_dir(directory)?
        .map(|res_entry| res_entry.map(|entry| entry.path()))
        .collect()
}

fn read_csv<P: AsRef<Path>>(path: P, schema: &Schema) -> PolarResult<DataFrame> {
    let file = File::open(path).expect("Cannot open file.");

    CsvReader::new(file)
        .with_schema(&Arc::new(schema))
        .has_header(true)
        .finish()
}

fn right_or_append(mut accumulator: DataFrame, right: DataFrame) -> PolarResult<DataFrame> {
    if accumulator.width() == 0 {
        Ok(right)
    } else {
        accumulator.vstack_mut(&right)?;
        Ok(accumulator)
    }
}

fn process_files_parallel(paths: &[PathBuf], schema: &Schema) -> PolarResult<DataFrame> {
    paths
        .into_par_iter()
        .map(|x| read_csv(x, schema))
        .try_reduce(DataFrame::default, right_or_append)
}

pub enum Schemas {
    Event,
    Project,
}

fn get_schema(schema: Schemas) -> Schema {
    match schema {
        Schemas::Event => Schema::new(vec![
            Field::new("address", DataType::Utf8),
            Field::new("event", DataType::Utf8),
            Field::new("topic1", DataType::Utf8),
            Field::new("topic2", DataType::Utf8),
            Field::new("topic3", DataType::Int64),
            Field::new("data", DataType::Utf8),
            Field::new("blockNumber", DataType::Int64),
            Field::new("transactionHash", DataType::Utf8),
            Field::new("transactionIndex", DataType::Int64),
            Field::new("logIndex", DataType::Int64),
            Field::new("transactionLogIndex", DataType::Utf8),
            Field::new("logType", DataType::Utf8),
            Field::new("removed", DataType::Boolean),
        ]),
        Schemas::Project => Schema::new(vec![
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
        ]),
    }
}

pub async fn get_events_df(
    address: &str,
    event: &str,
) -> Result<DataFrame, Box<dyn std::error::Error>> {
    let path = format!("./data/events/{}/{}", event, address);
    info!("{}", &path);

    let dataset_dir = PathBuf::from(path);
    let mut paths = read_dir(dataset_dir)?;
    paths.sort_unstable();

    let schema = get_schema(Schemas::Event);
    let df = paths
        .chunks(10)
        .try_fold(DataFrame::default(), |main_df, paths| {
            let df = process_files_parallel(paths, &schema)?;
            right_or_append(main_df, df)
        })?;
    info!(
        "Loaded {} records from {} files",
        df["blockNumber"].len(),
        paths.len(),
    );
    Ok(df)
}
