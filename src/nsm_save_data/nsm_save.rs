use std::fs;
use std::path::PathBuf;

use chrono::{DateTime, FixedOffset};

#[derive(Clone, Debug)]
pub struct NSMFile {
    pub name: String,
    pub save_date: DateTime<FixedOffset>,
}

impl NSMFile {
    pub fn to_string(&self) -> String {
        format!("{}\n{}", self.name, self.save_date.to_rfc3339())
    }
}

#[derive(Clone, Debug)]
pub struct NSMSave {
    pub name: String,
    pub save_date: DateTime<FixedOffset>,
    pub save_path: PathBuf,
}

impl NSMSave {
    pub fn to_nsm_file(&self) -> NSMFile {
        NSMFile {
            name: self.name.clone(),
            save_date: self.save_date,
        }
    }
}

pub fn read_nsm_file(path: &PathBuf) -> Result<NSMFile, std::io::Error> {
    match fs::read(path) {
        Ok(data) => {
            let lines: Vec<String> = String::from_utf8(data)
                .unwrap()
                .lines()
                .map(|l| l.trim().to_string())
                .collect();
            let nsm_save_info = NSMFile {
                name: lines[0].clone(),
                save_date: {
                    if lines.len() <= 1 {
                        DateTime::<FixedOffset>::parse_from_rfc3339("1970-01-01T00:00:00+00:00")
                            .unwrap()
                    } else {
                        match DateTime::parse_from_rfc3339(&lines[1]) {
                            Ok(date) => date,
                            Err(_) => DateTime::<FixedOffset>::parse_from_rfc3339(
                                "1970-01-01T00:00:00+00:00",
                            )
                            .unwrap(),
                        }
                    }
                },
            };
            Ok(nsm_save_info)
        }
        Err(e) => Err(e),
    }
}

pub fn is_valid_nsm_save_name(name: &str) -> bool {
    if name == "save00" {
        return false;
    }
    for c in name.chars() {
        if !(c.is_ascii_alphanumeric() || c == '-' || c == '_') {
            return false;
        }
    }
    true
}
