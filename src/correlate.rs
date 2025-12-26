use chrono::{DateTime, Duration, Utc};

use crate::model::Event;

#[derive(Debug, Clone)]
pub struct Incident {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub events: Vec<Event>,
}

pub fn group_incidents(events: &[Event], window_seconds: i64) -> Vec<Incident> {
    if events.is_empty() {
        return Vec::new();
    }

    let window = Duration::seconds(window_seconds);

    let mut incidents: Vec<Incident> = Vec::new();
    let mut current: Vec<Event> = Vec::new();

    for e in events.iter().cloned() {
        if current.is_empty() {
            current.push(e);
            continue;
        }

        let last_ts = current.last().unwrap().ts;
        if e.ts - last_ts <= window {
            current.push(e);
        } else {
            let start = current.first().unwrap().ts;
            let end = current.last().unwrap().ts;
            incidents.push(Incident {
                start,
                end,
                events: current,
            });
            current = vec![e];
        }
    }
    // flush final incident
    let start = current.first().unwrap().ts;
    let end = current.last().unwrap().ts;
    incidents.push(Incident {
        start,
        end,
        events: current,
    });
    incidents
}
