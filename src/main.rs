mod cli;
mod correlate;
mod generator;
mod model;
mod parse;
mod report;

use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    let cli = cli::Cli::parse();

    println!("{cli:?}");

    match cli.command {
        cli::Commands::Gen {
            out,
            seed,
            minutes,
            base_rate,
            incident_count,
        } => {
            println!(
                "gen -> out={out:?} seed={seed:?} minutes={minutes} base_rate={base_rate} incident_count={incident_count}"
            );

            generator::generate(&out, seed, minutes, base_rate, incident_count)?;
        }

        cli::Commands::Incidents {
            input,
            window,
            levels,
            format,
        } => {
            println!(
                "incidents -> input={input:?} window={window} levels={levels:?} format={format}"
            );
            let events = parse::parse_jsonl("sample_data/generated.jsonl")?;
            println!("Loaded {} events", events.len());

            let incidents = correlate::group_incidents_by_level(&events, 300);
            println!("Incidents found: {}", incidents.len());

            for (i, inc) in incidents.iter().enumerate() {
                let summary = report::summarize(inc);
                report::print_summary(i + 1, &summary);
            }
        }
    }

    /*


        /*
            // print the first 3 to check parsing
            for e in events.iter().take(3) {
                println!("{:?}", e);
            }
        */


        /* // Print out incident headers
            for (idx, inc) in incidents.iter().enumerate() {
                println!(
                    "Incidient # {}: {} -> {} ({} events)",
                    idx + 1,
                    inc.start,
                    inc.end,
                    inc.events.len()
                );
            }
        */
    */

    Ok(())
}
