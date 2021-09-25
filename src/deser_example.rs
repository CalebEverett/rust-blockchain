use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let fields: Vec<&str> = vec![
        "id",
        "artistName",
        "curationStatus",
        "dynamic",
        "name",
        "scriptJSON",
    ];

    let mut projects = get_projects(fields).await?;
    // let deserialized: Data = serde_json::from_str(&projects).unwrap();
    fn deser_script_string(record: &mut Record) -> () {
        record.script_json = serde_json::from_str(&record.script_json_string).unwrap();
    }

    projects.iter_mut().for_each(|r| deser_script_string(r));

    println!("{:?}", projects);
    Ok(())
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct ScriptJSON {
    #[serde(rename = "type")]
    script_type: String,
    version: String,
}

impl Default for ScriptJSON {
    fn default() -> Self {
        ScriptJSON {
            script_type: "".into(),
            version: "0.0.0".into(),
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct Record {
    id: String,
    name: String,
    #[serde(rename = "artistName")]
    artist_name: String,
    #[serde(rename = "curationStatus")]
    curation_status: String,
    dynamic: bool,
    #[serde(rename = "scriptJSON")]
    script_json_string: String,
    #[serde(skip_deserializing)]
    script_json: ScriptJSON,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct Projects {
    projects: Vec<Record>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct Data {
    data: Projects,
}

async fn get_projects<'a>(fields: Vec<&str>) -> Result<Vec<Record>, Box<dyn std::error::Error>> {
    let query = format!(
        "query {{
            projects(first: 2) {{
                {fields}
            }}
        }}",
        fields = fields.join("\n")
    );

    let mut params = HashMap::new();
    params.insert("query", &query);
    let client = reqwest::Client::new();
    let resp = client
        .post("https://api.thegraph.com/subgraphs/name/artblocks/art-blocks")
        .json(&params)
        .send()
        .await?
        .json::<Data>()
        .await?;

    Ok(resp.data.projects)
}
