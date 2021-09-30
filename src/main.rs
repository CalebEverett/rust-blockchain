use clap::{value_t, App, Arg, SubCommand};
use dotenv::dotenv;
use log::info;

mod dataframes;
mod events;
mod events_old;
mod projects;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    env_logger::init();

    let matches = App::new("nftylytics")
        .subcommand(
            SubCommand::with_name("projects")
                .about("Commands related to fetching, writing and reading projects")
                .arg(
                    Arg::with_name("num")
                        .long("num")
                        .short("n")
                        .help("Sets number of projects to retrieve")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("path")
                        .long("path")
                        .short("p")
                        .help("Sets write path")
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("events_old")
                .about("Commands related to fetching, writing and reading events")
                .arg(
                    Arg::with_name("num")
                        .long("num")
                        .short("n")
                        .help("Sets number of projects to retrieve")
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("events")
                .about("Commands related to fetching, writing and reading contracts")
                .arg(
                    Arg::with_name("address")
                        .long("address")
                        .short("a")
                        .help("Address of the contract to filter by")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("event")
                        .long("event")
                        .short("e")
                        .help("Name of the event to retrieve")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("from_block")
                        .long("from_block")
                        .short("f")
                        .help("Starting block for query")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("to_block")
                        .long("to_block")
                        .short("t")
                        .help("Ending block for query")
                        .takes_value(true),
                ),
        )
        .get_matches();
    match matches.subcommand() {
        ("projects", Some(m)) => {
            let num = value_t!(m, "num", u16).unwrap_or(200);

            let path = m.value_of("path").unwrap_or("projects.csv");

            println!("Fetching {} project records and writing to {}", &num, &path);
            projects::write_csv(&path, num).await?;
            let df = dataframes::get_projects_df(&path).await?;

            info!("{:?}", df);
        }
        ("events_old", Some(m)) => {
            events_old::write_csv(&"hello").await?;
        }
        ("events", Some(m)) => {
            // "0x059EDD72Cd353dF5106D2B9cC5ab83a52287aC3a"
            // 11341548, 11351548
            let address = m.value_of("address").unwrap();
            let event = m.value_of("event").unwrap();
            let from_block = value_t!(m, "from_block", u64)?;
            let to_block = value_t!(m, "to_block", u64)?;
            info!(
                "address: {}, event: {}, from_block: {}, to_block: {}",
                &address, &event, &from_block, &to_block
            );
            events::get_events_all(address, event, from_block, to_block).await?;
        }
        _ => println!("Other command received"),
    }
    Ok(())
}
