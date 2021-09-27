use std::collections::HashMap;

pub async fn write_csv(path: &str, num: u16) -> Result<(), Box<dyn std::error::Error>> {
    let fields: Vec<&str> = vec![
        "id",
        "projectId",
        "name",
        "artistName",
        "curationStatus",
        "invocations",
        "maxInvocations",
        "dynamic",
        "scriptJSON",
        "website",
        "license",
        "active",
        "paused",
    ];

    let records = get_projects(&fields, num).await?;
    let mut wtr = csv::Writer::from_path(path)?;
    // wtr.write_record(&fields)?;
    for r in &records {
        wtr.serialize(r)?;
    }
    wtr.flush()?;
    Ok(())
}

pub mod schema {
    use serde_with::DisplayFromStr;

    #[serde_with::serde_as]
    #[derive(serde::Serialize, serde::Deserialize, Debug)]
    struct ScriptJSON {
        #[serde(rename = "type")]
        script_type: String,
        // #[serde_as(as = "DisplayFromStr")]
        // interactive: Option<bool>,
        version: String,
    }

    impl Default for ScriptJSON {
        fn default() -> Self {
            ScriptJSON {
                script_type: "".into(),
                // interactive: Some(true),
                version: "0.0.0".into(),
            }
        }
    }

    #[serde_with::serde_as]
    #[derive(serde::Serialize, serde::Deserialize, Debug)]
    pub struct Record {
        id: Option<String>,
        #[serde(rename = "projectId")]
        #[serde_as(as = "DisplayFromStr")]
        project_id: u32,
        name: Option<String>,
        #[serde(rename = "artistName")]
        artist_name: Option<String>,
        #[serde(rename = "curationStatus")]
        curation_status: Option<String>,
        #[serde_as(as = "DisplayFromStr")]
        invocations: u32,
        #[serde(rename = "maxInvocations")]
        #[serde_as(as = "DisplayFromStr")]
        max_invocations: u32,
        dynamic: bool,
        #[serde(rename = "scriptJSON")]
        script_json_string: Option<String>,
        #[serde(skip_serializing)]
        #[serde(skip_deserializing)]
        script_json: ScriptJSON,
        website: Option<String>,
        license: Option<String>,
        active: bool,
        paused: bool,
    }

    #[derive(serde::Serialize, serde::Deserialize, Debug)]
    pub struct Projects {
        pub projects: Vec<Record>,
    }

    #[derive(serde::Serialize, serde::Deserialize, Debug)]
    pub struct Data {
        pub data: Projects,
    }
}

async fn get_projects(
    fields: &Vec<&str>,
    num: u16,
) -> Result<Vec<schema::Record>, Box<dyn std::error::Error>> {
    let query = format!(
        "query {{
            projects(first: {num}) {{
                {fields}
            }}
        }}",
        fields = fields.join("\n"),
        num = num
    );

    let mut params = HashMap::new();
    params.insert("query", &query);
    let client = reqwest::Client::new();
    let resp = client
        .post("https://api.thegraph.com/subgraphs/name/artblocks/art-blocks")
        .json(&params)
        .send()
        .await?
        .json::<schema::Data>()
        .await?;

    Ok(resp.data.projects)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
