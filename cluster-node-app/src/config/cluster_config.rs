use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ClusterNodeConfig {
    pub id: u64,
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct ClusterConfig {
    #[serde(rename = "clusterName")]
    pub cluster_name: String,
    pub nodes: Vec<ClusterNodeConfig>,
}

impl ClusterConfig {
    pub fn from_file(file_path: &str) -> Result<Self, std::io::Error> {
        let file = File::open(file_path)?;
        let mut buffered_reader = BufReader::new(file);
        let mut contents = String::new();
        buffered_reader.read_to_string(&mut contents)?;

        serde_json::from_str::<Self>(contents.as_str()).map_err(to_deserialize_error)
    }
}

fn to_deserialize_error(e: serde_json::Error) -> std::io::Error {
    std::io::Error::new(
        std::io::ErrorKind::Other,
        format!("File Config Deserialize Error: {:?}", e),
    )
}
