use std::fs;
use std::io::{self, Read, Write};

use anyhow::{Context, Result};
use clap::Parser;

use basic_to_text::{BasicVersion, decode};

#[derive(Parser)]
#[command(name = "basic-to-text", about = "Detokenise BBC BASIC files to text")]
struct Cli {
    /// Input file (reads from stdin if omitted)
    input: Option<String>,

    /// Output file (writes to stdout if omitted)
    #[arg(short, long)]
    output: Option<String>,

    /// Use BBC BASIC 2 token set
    #[arg(long, group = "version")]
    basic2: bool,

    /// Use BBC BASIC V token set (default)
    #[arg(long, group = "version")]
    basicv: bool,

    /// Prefix lines with line numbers
    #[arg(short = 'n', long)]
    line_numbers: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let version = if cli.basic2 {
        BasicVersion::Basic2
    } else {
        BasicVersion::BasicV
    };

    let data = match &cli.input {
        Some(path) => fs::read(path).with_context(|| format!("reading {path}"))?,
        None => {
            let mut buf = Vec::new();
            io::stdin().read_to_end(&mut buf).context("reading stdin")?;
            buf
        }
    };

    let text = decode(&data, version, cli.line_numbers)?;

    match &cli.output {
        Some(path) => {
            fs::write(path, &text).with_context(|| format!("writing {path}"))?;
        }
        None => {
            io::stdout()
                .write_all(text.as_bytes())
                .context("writing stdout")?;
        }
    }

    Ok(())
}
