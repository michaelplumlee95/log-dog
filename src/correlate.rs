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

fn is_incident_level(level: &str) -> bool {
    matches!(level, "WARN" | "ERROR")
}

pub fn group_incidents_by_level(events: &[Event], window_seconds: i64) -> Vec<Incident> {
    let filtered: Vec<Event> = events
        .iter()
        .filter(|e| is_incident_level(e.level.as_str()))
        .cloned()
        .collect();

    group_incidents(&filtered, window_seconds)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::Event;

    fn ev(ts: &str, level: &str) -> Event {
        Event {
            ts: ts.parse().expect("valid RFC3339 timestamp"),
            system: "power".to_string(),
            level: level.to_string(),
            msg: "x".to_string(),
            host: Some("dc01".to_string()),
            code: Some("CODE".to_string()),
        }
    }

    #[test]
    fn empty_input_produces_no_incidents() {
        let incidents = group_incidents(&[], 300);
        assert_eq!(incidents.len(), 0);
    }

    #[test]
    fn events_within_window_cluster_into_one_incident() {
        let events = vec![
            ev("2025-12-26T10:00:00Z", "ERROR"),
            ev("2025-12-26T10:04:59Z", "WARN"), // 299s later
        ];

        let incidents = group_incidents(&events, 300);
        assert_eq!(incidents.len(), 1);
        assert_eq!(incidents[0].events.len(), 2);
        assert_eq!(incidents[0].start.to_rfc3339(), "2025-12-26T10:00:00+00:00");
    }

    #[test]
    fn events_outside_window_split_into_two_incidents() {
        let events = vec![
            ev("2025-12-26T10:00:00Z", "ERROR"),
            ev("2025-12-26T10:05:01Z", "ERROR"), // 301s gap
        ];

        let incidents = group_incidents(&events, 300);
        assert_eq!(incidents.len(), 2);
        assert_eq!(incidents[0].events.len(), 1);
        assert_eq!(incidents[1].events.len(), 1);
    }

    #[test]
    fn filtering_levels_excludes_info_events() {
        let events = vec![
            ev("2025-12-26T10:00:00Z", "INFO"),
            ev("2025-12-26T10:01:00Z", "ERROR"),
            ev("2025-12-26T10:02:00Z", "INFO"),
            ev("2025-12-26T10:03:00Z", "WARN"),
        ];

        let incidents = group_incidents_by_level(&events, 300);

        assert_eq!(incidents.len(), 1);
        assert_eq!(incidents[0].events.len(), 2);

        let levels: Vec<&str> = incidents[0]
            .events
            .iter()
            .map(|e| e.level.as_str())
            .collect();

        assert!(levels.contains(&"ERROR"));
        assert!(levels.contains(&"WARN"));
        assert!(!levels.contains(&"INFO"));
    }
}
