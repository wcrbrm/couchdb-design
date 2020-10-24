use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use super::designdoc::DesignDoc;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Compare {
    pub added: Vec<String>,
    pub deleted: Vec<String>,
    pub not_modified: Vec<String>,
    pub modified: HashMap<String, String>,
}

impl fmt::Display for Compare {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut arr = vec![];
        if self.added.len() > 0 {
            arr.push(format!("NEW: {} views", self.added.join(",")));
        }
        if self.deleted.len() > 0 {
            arr.push(format!("DEL: {} views", self.deleted.join(",")));
        }
        if self.not_modified.len() > 0 {
            arr.push(format!("NOT MODIFIED: {} views", self.not_modified.len()));
        }
        if self.modified.len() > 0 {
            arr.push(format!("MODIFIED: {} views", self.modified.len()));
        }
        if arr.len() > 0 {
            return write!(f, "{}", arr.join(","));
        }
        Ok(())
    }
}

impl Compare {
    pub fn docs(src: &DesignDoc, origin: &DesignDoc) -> Compare {
        let mut res = Compare {
            added: vec![],
            deleted: vec![],
            not_modified: vec![],
            modified: HashMap::new(),
        };
        for (k, _) in &origin.views {
            if let None = src.views.get(k) {
                res.deleted.push(k.clone());
            }
        }
        for (k, v) in &src.views {
            match origin.views.get(k) {
                None => res.added.push(k.clone()),
                Some(origin_val) => {
                    if v.to_string().trim() != origin_val.to_string().trim() {
                        res.modified.insert(k.clone(), v.to_string());
                    } else {
                        res.not_modified.push(k.clone());
                    }
                }
            }
        }
        res
    }
}