extern crate term;
use std::error::Error;
use std::collections::HashMap;
use std::fmt;
use serde::{Deserialize, Serialize};
use difference::{ Changeset, Difference };
use super::designdoc::DesignDoc;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CompareStrings {
    pub old_str: String,
    pub new_str: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Compare {
    pub added: Vec<String>,
    pub deleted: Vec<String>,
    pub not_modified: Vec<String>,
    pub modified: HashMap<String, CompareStrings>,
}

impl fmt::Display for Compare {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut arr = vec![];
        if self.added.len() > 0 {
            arr.push(format!("NEW: {}", self.added.join(",")));
        }
        if self.deleted.len() > 0 {
            arr.push(format!("DELETED: {}", self.deleted.join(",")));
        }
        if self.not_modified.len() > 0 {
            arr.push(format!("NOT MODIFIED: {} views", self.not_modified.len()));
        }
        if self.modified.len() > 0 {
            arr.push(format!("MODIFIED: {} views", self.modified.len()));
        }
        if arr.len() > 0 {
            return write!(f, "{}", arr.join(", "));
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
                        res.modified.insert(k.clone(), CompareStrings{ 
                            new_str: v.to_string().trim().to_string(),
                            old_str: origin_val.to_string().trim().to_string(),
                        });
                    } else {
                        res.not_modified.push(k.clone());
                    }
                }
            }
        }
        res
    }
    pub fn is_modified(&self) -> bool {
        self.added.len() + self.deleted.len() + self.modified.len() > 0
    }
    
    pub fn show_details(&self) -> Result<(), Box<dyn Error>> {
        for (k, v) in &self.modified {
            println!("{}", k.to_uppercase());
            let changeset = Changeset::new(v.old_str.as_str(), v.new_str.as_str(), "\n");
            let mut t = term::stdout().unwrap();
            let diffs = changeset.diffs;
            for i in 0..diffs.len() {
                match diffs[i] {
                    Difference::Same(ref x) => {
                        t.reset().unwrap();
                        writeln!(t, " {}", x)?;
                    }
                    Difference::Add(ref x) => {
                        t.fg(term::color::GREEN).unwrap();
                        writeln!(t, "+{}", x)?;
                    }
                    Difference::Rem(ref x) => {
                        t.fg(term::color::RED).unwrap();
                        writeln!(t, "-{}", x)?;
                    }
                }
            }
            t.reset().unwrap();
            t.flush().unwrap();
        }
        Ok(())
    }
}
