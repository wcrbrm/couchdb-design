use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::path::PathBuf;

fn prefixed(s: &String, prefix: &str) -> String {
    s.trim()
        .lines()
        .filter(|f| f.trim().len() > 0)
        .map(|f| format!("{}{}\n", prefix, f.trim_end()))
        .collect()
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CouchDbErrorMessage {
    pub error: String,
    pub reason: String,
}

impl fmt::Display for CouchDbErrorMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "error: {} reason: {}", self.error, self.reason)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DesignView {
    map: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    reduce: Option<String>,
}

impl fmt::Display for DesignView {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "map: |\n{}", prefixed(&self.map, "  ").as_str())?;
        if let Some(re) = &self.reduce {
            write!(f, "reduce: {}", &re.as_str())?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DesignDoc {
    pub _id: String,
    pub _rev: Option<String>,
    pub language: Option<String>,
    pub views: HashMap<String, DesignView>,
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DesignDocSubmitted {
    // pub ok: bool,
    pub id: String,
    pub rev: String,
}

impl fmt::Display for DesignDoc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "---")?;
        writeln!(f, "_id: {:?}", &self._id)?;
        if let Some(some_language) = &self.language {
            writeln!(f, "language: {:?}", some_language)?;
        }
        writeln!(f, "views:")?;
        for (key, value) in &self.views {
            let str_val = format!("{}", value);
            write!(f, "  {}:\n{}", key, prefixed(&str_val, "    "))?;
        }
        Ok(())
    }
}

impl DesignDoc {
    pub async fn from_url(url: &str) -> Result<Option<DesignDoc>, Box<dyn Error>> {
        let response = reqwest::get(url).await?;
        if response.status().is_success() {
            let document = response.json::<DesignDoc>().await?;
            return Ok(Some(document));
        }
        let err_doc = response.json::<CouchDbErrorMessage>().await?;
        Err(format!("{} URL: {}", err_doc, url).into())
    }

    pub async fn from_file(filepath: PathBuf) -> Result<DesignDoc, Box<dyn Error>> {
        let file = File::open(filepath).expect("file could not be open");
        let document: DesignDoc = serde_yaml::from_reader(file).expect("YAML file could not be read");
        Ok(document)
    }

    #[allow(dead_code)]
    pub async fn from_stdout() -> Result<DesignDoc, Box<dyn Error>> {
        let document: DesignDoc = serde_yaml::from_reader(std::io::stdin())?;
        Ok(document)
    }

    pub async fn submit(doc: &DesignDoc, url: &str) -> Result<DesignDocSubmitted, Box<dyn Error>> {
        let rq = reqwest::Client::new().put(url);
        let response = rq
            .header("Content-Type", "application/json")
            .body(serde_json::to_string(&doc)?)
            .send()
            .await?;
        if response.status().is_success() {
            let document = response.json::<DesignDocSubmitted>().await?;
            return Ok(document);
        }
        let err_doc = response.json::<CouchDbErrorMessage>().await?;
        Err(format!("{} URL: {}", err_doc, url).into())
    }

    pub async fn create(&self, url: &str) -> Result<DesignDocSubmitted, Box<dyn Error>> {
        let mut doc = self.clone();
        doc._rev = None;
        DesignDoc::submit(&doc, url).await
    }

    pub async fn update(&self, url: &str) -> Result<DesignDocSubmitted, Box<dyn Error>> {
        DesignDoc::submit(&self, url).await
    }
}
