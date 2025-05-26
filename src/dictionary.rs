use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub struct Dictionary {
    pub words: Vec<String>,
}

impl Dictionary {
    pub fn load_from_file<P>(path: P) -> Result<Dictionary, std::io::Error> 
    where 
        P: AsRef<Path> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let words = reader
            .lines()
            .filter_map(|line| {
                line.ok()
                    .map(|s| s.trim().to_lowercase())
                    .filter(|s| !s.is_empty())
            })
            .collect();

        Ok(Dictionary {words})
    }
}
