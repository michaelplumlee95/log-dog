use chrono::{DateTime, Duration, Utc};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng, seq::SliceRandom};
use serde::Serialize;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

#[derive(Serialize)]
struct LogLine<'a> {
    ts: DateTime<Utc>,
    system: &'a str,
    level: &'a str,
    msg: &'a str,
    host: &'a str,
    code: &'a str,
}

pub fn generate(
    out_path: &Path,
    seed: Option<u64>,
    minutes: i64,
    base_rate: u32,
    incident_count: u32,
) -> anyhow::Result<()> {
    let mut rng: StdRng = match seed {
        Some(s) => StdRng::seed_from_u64(s),
        None => StdRng::from_entropy(),
    };

    // Configs

    let systems = ["hvac", "power", "cooling", "monitoring"];
    let hosts = ["dc01", "dc02", "dc03"];

    let baseline = [
        ("INFO", "Routine status check", "STATUS_OK"),
        ("INFO", "Operating normally", "OK"),
        ("INFO", "No active alarms", "CLEAR"),
        ("INFO", "Minor fluctuation detected", "FLUX"),
    ];

    let incident_events = [
        ("WARN", "Voltage dip detected", "VOLT_DIP", "power"),
        ("ERROR", "UPS on battery", "UPS_BATT", "power"),
        ("ERROR", "Generator start failure", "GEN_FAIL", "power"),
        ("ERROR", "Critical load shed", "LOAD_SHED", "power"),
        ("WARN", "Multiple alarms active", "MULTI", "monitoring"),
        ("WARN", "Supply air temperature high", "TEMP_HIGH", "hvac"),
        ("ERROR", "Compressor trip detected", "COMP_TRIP", "hvac"),
        ("WARN?", "Chilled water flow reduced", "FLOW_LOW", "cooling"),
    ];

    let start = Utc::now() - Duration::minutes(minutes);
    let mut lines: Vec<LogLine> = Vec::new();

    // Baseline noise
    for m in 0..minutes {
        let t0 = start + Duration::minutes(m);

        let n = rng.gen_range(0..=base_rate * 2);

        for _ in 0..n {
            let (level, msg, code) = baseline.choose(&mut rng).unwrap();
            let system = systems.choose(&mut rng).unwrap();
            let host = hosts.choose(&mut rng).unwrap();

            let ts = t0 + Duration::seconds(rng.gen_range(0..60));

            lines.push(LogLine {
                ts,
                system,
                level,
                msg,
                host,
                code,
            });
        }
    }

    // Incident burst

    for _ in 0..incident_count {
        let burst_start_min = rng.gen_range(10..minutes - 10);
        let burst_start = start + Duration::minutes(burst_start_min);

        let burst_len_sec = rng.gen_range(120..420); // 2- 7 minute burst
        let burst_events = rng.gen_range(6..18);

        for _ in 0..burst_events {
            let (level, msg, code, forced_system) = incident_events.choose(&mut rng).unwrap();
            let host = hosts.choose(&mut rng).unwrap();
            let ts = burst_start + Duration::seconds(rng.gen_range(0..burst_len_sec));

            lines.push(LogLine {
                ts,
                system: forced_system,
                level,
                msg,
                host,
                code,
            });
        }
    }

    // Sort chronologically
    lines.sort_by_key(|l| l.ts);

    // Write JSONL
    std::fs::create_dir_all("sample_data")?;
    let file = File::create(out_path)?;
    let mut w = BufWriter::new(file);

    for l in lines {
        let json = serde_json::to_string(&l)?;
        writeln!(w, "{json}")?;
    }

    println!("Wrote generated logs.");
    Ok(())
}
