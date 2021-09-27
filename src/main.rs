use clap::{value_t, App, Arg};

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

mod dataframes;
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
    let df = dataframes::get_projects_df(&path).await?;

    println!("{:?}", df);
    Ok(())
}
