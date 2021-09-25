// use polars::df;
// use polars::frame::DataFrame;
// use polars::prelude::NamedFrom;
// use polars::series::Series;
use polars::prelude::CsvReader;
use polars::prelude::SerReader;

mod projects;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let records = projects::records(2, true).await?;

    // let ids: Vec<Option<String>> = records.iter().map(|r| r.id.as_ref()).collect();
    // let project_ids: Vec<u32> = records.iter().map(|r| r.project_id).collect();
    // let df = df![
    //     // "id" => ids,
    //     "project_id" => project_ids

    // ]?;

    let df = CsvReader::from_path("projects.csv")?
        .infer_schema(None)
        .has_header(true)
        .finish()?;

    println!("{:?}", df);
    Ok(())
}
