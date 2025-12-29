use crate::correlate::Incident;
use chrono::Duration;
use std::collections::{BTreeMap, BTreeSet};

pub struct IncidentSummary {
    pub start: String,
    pub end: String,
    pub duration: Duration,
    pub total: usize,
    pub counts_by_level: BTreeMap<String, usize>,
    pub systems: BTreeSet<String>,
    pub hosts: BTreeSet<String>,
    pub codes: BTreeMap<String, usize>,
    pub msg_preview: Vec<String>,
}

pub fn summarize(inc: &Incident) -> IncidentSummary {
    let mut counts_by_level: BTreeMap<String, usize> = BTreeMap::new();
    let mut systems: BTreeSet<String> = BTreeSet::new();
    let mut hosts: BTreeSet<String> = BTreeSet::new();
    let mut codes: BTreeMap<String, usize> = BTreeMap::new();
    let mut msg_preview: Vec<String> = Vec::new();

    for e in &inc.events {
        *counts_by_level.entry(e.level.clone()).or_insert(0) += 1;
        systems.insert(e.system.clone());

        if let Some(h) = &e.host {
            hosts.insert(h.clone());
        }

        if let Some(c) = &e.code {
            *codes.entry(c.clone()).or_insert(0) += 1;
        }

        if msg_preview.len() < 3 {
            msg_preview.push(format!("{} {}: {}", e.level, e.system, e.msg))
        }
    }

    IncidentSummary {
        start: inc.start.to_rfc3339(),
        end: inc.end.to_rfc3339(),
        duration: inc.end - inc.start,
        total: inc.events.len(),
        counts_by_level,
        systems,
        hosts,
        codes,
        msg_preview,
    }
}

pub fn print_summary(idx: usize, s: &IncidentSummary) {
    let secs = s.duration.num_seconds();
    let mins = secs / 60;
    let rem = secs % 60;

    let info = s.counts_by_level.get("INFO").copied().unwrap_or(0);
    let warn = s.counts_by_level.get("WARN").copied().unwrap_or(0);
    let error = s.counts_by_level.get("ERROR").copied().unwrap_or(0);

    println!(
        "Incident #{idx} {} → {} ({}m{}s)",
        s.start, s.end, mins, rem
    );

    println!(
        "   events: {}  |   ERROR: {}   WARN: {}    INFO {}",
        s.total, error, warn, info
    );

    let systems = s.systems.iter().cloned().collect::<Vec<_>>().join(", ");
    println!(
        "   systems: {}",
        if systems.is_empty() {
            "(none)".into()
        } else {
            systems
        }
    );
    let hosts = s.hosts.iter().cloned().collect::<Vec<_>>().join(", ");
    println!(
        "   hosts: {}",
        if hosts.is_empty() {
            "(none)".into()
        } else {
            hosts
        }
    );

    // top 5 codes

    let mut code_vec: Vec<(&String, &usize)> = s.codes.iter().collect();
    code_vec.sort_by(|a, b| b.1.cmp(a.1).then_with(|| a.0.cmp(b.0)));

    println!("  sample messages:");
    for line in &s.msg_preview {
        println!("      - {line}");
    }

    println!("  top codes:");
    for (code, count) in code_vec.into_iter().take(5) {
        println!("      - {} ({})", code, count);
    }

    println!();
}
