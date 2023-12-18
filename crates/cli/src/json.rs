use std::{time::Duration, collections::HashMap};

#[derive(Debug, Clone, Copy, serde::Deserialize)]
pub struct Stats {
    pub samples: usize,
    pub min: Duration,
    pub max: Duration,
    pub mean: Duration,
    pub std_dev: Duration,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub enum Summary {
    Run(Vec<RunSummary>),
    Bench(Vec<BenchSummary>),
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct RunSummary {
    pub name: String,
    pub result: String,
    pub time: Duration,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct BenchSummary {
    pub name: String,
    pub stats: Stats,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DayMeta {
    pub name: Option<String>,
    pub answer1: Option<String>,
    pub answer2: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AllMetadata {
    pub days: HashMap<String, DayMeta>
}