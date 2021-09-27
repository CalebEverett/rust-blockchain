use clap::{value_t, App, Arg};
use polars::prelude::{CsvReader, DataType, Field, Schema, SerReader};

// "id",
// "projectId",
// "name",
// "artistName",
// "curationStatus",
// "invocations",
// "maxInvocations",
// "dynamic",
// "scriptJSON",
// "website",
// "license",
// "active",
// "paused",

mod projects;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("Fetching Project Data")
        .arg(
            Arg::with_name("num")
                .short("n")
                .help("Sets number of projects to retrieve")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("path")
                .short("p")
                .help("Sets path to write file to")
                .takes_value(true),
        )
        .get_matches();
    let num = value_t!(matches, "num", u16).unwrap_or(200);

    let path = matches.value_of("path").unwrap_or("projects.csv");

    println!("Fetching {} project records and writing to {}", &num, &path);
    projects::write_csv(&path, num).await?;

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
        .finish()?;

    println!("{:?}", df);
    Ok(())
}
