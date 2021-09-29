use clap::{value_t, App, Arg, SubCommand};
use dotenv::dotenv;

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

mod contracts;
mod dataframes;
mod events;
mod projects;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let matches = App::new("NFT ")
        .subcommand(
            SubCommand::with_name("projects")
                .about("Commands related to fetching, writing and reading projects")
                .arg(
                    Arg::with_name("num")
                        .short("n")
                        .help("Sets number of projects to retrieve")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("path")
                        .short("p")
                        .help("Sets write path")
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("events")
                .about("Commands related to fetching, writing and reading events")
                .arg(
                    Arg::with_name("num")
                        .short("n")
                        .help("Sets number of projects to retrieve")
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("contracts")
                .about("Commands related to fetching, writing and reading contracts"),
        )
        .get_matches();
    match matches.subcommand_name() {
        Some("projects") => {
            let num = value_t!(matches, "num", u16).unwrap_or(200);

            let path = matches.value_of("path").unwrap_or("projects.csv");

            println!("Fetching {} project records and writing to {}", &num, &path);
            projects::write_csv(&path, num).await?;
            let df = dataframes::get_projects_df(&path).await?;

            println!("{:?}", df);
        }
        Some("events") => {
            events::write_csv(&"hello").await?;
        }
        Some("contracts") => {
            contracts::get_contract().await?;
        }
        None => println!("No commands received"),
        _ => println!("Other command received"),
    }
    Ok(())
}
