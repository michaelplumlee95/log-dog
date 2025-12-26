mod correlate;
mod model;
mod parse;

use anyhow::Result;

fn main() -> Result<()> {
    let events = parse::parse_jsonl("sample_data/logs.jsonl")?;
    println!("Loaded {} events", events.len());

    /*
        // print the first 3 to check parsing
        for e in events.iter().take(3) {
            println!("{:?}", e);
        }
    */

    let incidents = correlate::group_incidents(&events, 300);

    for (idx, inc) in incidents.iter().enumerate() {
        println!(
            "Incidient # {}: {} -> {} ({} events)",
            idx + 1,
            inc.start,
            inc.end,
            inc.events.len()
        );
    }

    Ok(())
}
