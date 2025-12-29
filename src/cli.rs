use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "log-dog", version, about = "Correlate logs into incidents")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Generate synthetic JSONL logs
    Gen {
        /// output path for generated file
        #[arg(short, long, default_value = "sample_data/generated.jsonl")]
        out: PathBuf,

        /// RNG seed for deterministic generation
        #[arg(long)]
        seed: Option<u64>,

        /// Total time span in minutes
        #[arg(long, default_value_t = 180)]
        minutes: i64,

        /// Average noise events per minutes
        #[arg(long, default_value_t = 2)]
        base_rate: u32,

        /// Number of incident bursts to inject
        #[arg(long, default_value_t = 3)]
        incident_count: u32,
    },

    /// Parse logs and group into incidents
    Incidents {
        /// Input JSONL file
        #[arg(short, long, default_value = "sample_data/generated.jsonl")]
        input: PathBuf,

        /// Incident window in seconds
        #[arg(short, long, default_value_t = 300)]
        window: i64,

        /// Comma-Separated list of levels that trigger incidents
        #[arg(long, value_delimiter = ',', default_value = "WARN,ERROR")]
        levels: Vec<String>,

        #[arg(long, default_value = "text")]
        format: String,
    },
}
