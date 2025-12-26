use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::{Context, Result};

use crate::model::Event;

pub fn parse_jsonl(path: &str) -> Result<Vec<Event>> {
    let file = File::open(path).with_context(|| format!("failed to open {path}"))?;
    let reader = BufReader::new(file);

    let mut events = Vec::new();

    for (i, line_result) in reader.lines().enumerate() {
        let line = line_result.with_context(|| format!("failed to read line {}", i + 1))?;
        if line.trim().is_empty() {
            continue;
        }

        let event: Event = serde_json::from_str(&line)
            .with_context(|| format!("invalid JSON on line {}: {}", i + 1, line))?;
        events.push(event);
    }

    events.sort_by_key(|e| e.ts);
    Ok(events)
}
