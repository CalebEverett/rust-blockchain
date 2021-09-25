mod projects;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let records = projects::records().await;
    println!("{:?}", records);
    Ok(())
}
