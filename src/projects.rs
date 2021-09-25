use std::collections::HashMap;

pub async fn records(
    num: u16,
    write: bool,
) -> Result<Vec<schema::Record>, Box<dyn std::error::Error>> {
    let fields: Vec<&str> = vec![
        "id",
        "projectId",
        "active",
        "artistName",
        "curationStatus",
        "dynamic",
        "invocations",
        "license",
        "maxInvocations",
        "name",
        "paused",
        "scriptJSON",
        "website",
    ];

    let records = get_projects(&fields, num).await?;
    // let deserialized: Data = serde_json::from_str(&projects).unwrap();
    // fn deser_script_string(record: &mut Record) -> () {
    //     record.script_json = serde_json::from_str(&record.script_json_string).unwrap();
    // }

    // projects.iter_mut().for_each(|r| deser_script_string(r));

    // println!("{:?}", projects);
    if write {
        let mut wtr = csv::Writer::from_path("projects.csv")?;
        wtr.write_record(&fields)?;
        for r in &records {
            wtr.serialize(r)?;
        }
        wtr.flush()?
    }

    Ok(records)
}

pub mod schema {
    use serde_with::DisplayFromStr;

    #[serde_with::serde_as]
    #[derive(serde::Serialize, serde::Deserialize, Debug)]
    pub struct ScriptJSON {
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
        pub id: Option<String>,
        #[serde(rename = "projectId")]
        #[serde_as(as = "DisplayFromStr")]
        pub project_id: u32,
        pub name: Option<String>,
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
