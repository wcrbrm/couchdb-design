use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;

fn prefixed(s: &String, prefix: &str) -> String {
    s.trim()
        .lines()
        .filter(|f| f.trim().len() > 0)
        .map(|f| format!("{}{}\n", prefix, f.trim_end()))
        .collect()
}

#[derive(Clone, Debug, Deserialize)]
struct DesignView {
    map: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    reduce: Option<String>,
}

impl fmt::Display for DesignView {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "map: |\n{}", prefixed(&self.map, "  ").as_str())?;
        if let Some(re) = &self.reduce {
            write!(f, "reduce: |\n\t|{}", prefixed(&re, "  ").as_str())?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize)]
struct DesignDocument {
    _id: String,
    _rev: Option<String>,
    language: Option<String>,
    views: HashMap<String, DesignView>,
}

impl fmt::Display for DesignDocument {
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

pub async fn display(url: String) -> Result<(), Box<dyn Error>> {
    let document = reqwest::get(url.as_str())
        .await?
        .json::<DesignDocument>()
        .await?;
    print!("{}", &document);
    Ok(())
}
