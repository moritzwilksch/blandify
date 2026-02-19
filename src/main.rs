use std::fs;
use std::io::{self, Read, Write};

use clap::{CommandFactory, Parser};

use blandify::{Categories, Normalizer, NormalizerConfig};

/// Unicode normalization tool for stripping LLM artifacts.
///
/// Normalizes smart quotes, dashes, zero-width characters, and other
/// Unicode oddities to plain ASCII equivalents.
#[derive(Parser)]
#[command(name = "blandify", version, about)]
struct Cli {
    /// Input file path or '-' for stdin
    input: Option<String>,

    /// Write to file instead of stdout
    #[arg(short, long)]
    output: Option<String>,

    /// Overwrite the input file in place
    #[arg(long, conflicts_with = "output")]
    in_place: bool,

    /// Disable smart quote normalization
    #[arg(long)]
    no_quotes: bool,

    /// Disable dash normalization
    #[arg(long)]
    no_dashes: bool,

    /// Disable Unicode whitespace normalization
    #[arg(long)]
    no_whitespace: bool,

    /// Disable zero-width character removal
    #[arg(long)]
    no_zero_width: bool,

    /// Disable arrow normalization
    #[arg(long)]
    no_arrows: bool,

    /// Disable fraction normalization
    #[arg(long)]
    no_fractions: bool,

    /// Disable math operator normalization
    #[arg(long)]
    no_math: bool,

    /// Disable symbol normalization
    #[arg(long)]
    no_symbols: bool,

    /// Enable German umlaut expansion (ä→ae, ö→oe, ü→ue, ß→ss)
    #[arg(long)]
    umlauts: bool,
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    if cli.in_place && cli.input.is_none() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "--in-place requires an input file path",
        ));
    }
    if cli.in_place && cli.input.as_deref() == Some("-") {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "--in-place cannot be used with stdin ('-')",
        ));
    }

    if cli.input.is_none() {
        let mut cmd = Cli::command();
        cmd.print_help()?;
        println!();
        return Ok(());
    }

    // Build categories from CLI flags
    let mut cats = Categories::DEFAULT;
    if cli.no_quotes {
        cats.remove(Categories::QUOTES);
    }
    if cli.no_dashes {
        cats.remove(Categories::DASHES);
    }
    if cli.no_whitespace {
        cats.remove(Categories::WHITESPACE);
    }
    if cli.no_zero_width {
        cats.remove(Categories::ZERO_WIDTH);
    }
    if cli.no_arrows {
        cats.remove(Categories::ARROWS);
    }
    if cli.no_fractions {
        cats.remove(Categories::FRACTIONS);
    }
    if cli.no_math {
        cats.remove(Categories::MATH);
    }
    if cli.no_symbols {
        cats.remove(Categories::SYMBOLS);
    }
    if cli.umlauts {
        cats.insert(Categories::UMLAUTS);
    }

    let config = NormalizerConfig::from_categories(cats);
    let normalizer = Normalizer::with_config(config);

    // Read input
    let input = match cli.input.as_deref() {
        Some("-") => read_stdin()?,
        Some(path) => fs::read_to_string(path)?,
        None => unreachable!("checked above: input is required"),
    };

    let output = normalizer.normalize(&input);

    // Write output
    if cli.in_place {
        let path = cli
            .input
            .as_deref()
            .expect("checked above: --in-place requires input");
        fs::write(path, &output)?;
    } else {
        match cli.output.as_deref() {
            Some("-") | None => write_stdout(output.as_bytes())?,
            Some(path) => fs::write(path, &output)?,
        }
    }

    Ok(())
}

fn read_stdin() -> io::Result<String> {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf)?;
    Ok(buf)
}

fn write_stdout(bytes: &[u8]) -> io::Result<()> {
    match io::stdout().write_all(bytes) {
        Ok(()) => Ok(()),
        Err(err) if err.kind() == io::ErrorKind::BrokenPipe => Ok(()),
        Err(err) => Err(err),
    }
}
