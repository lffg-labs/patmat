use core::Search;
use std::{
    fs::File,
    io::{self, Read, Result},
    path::PathBuf,
    time::Instant,
};

use clap::{Parser, ValueEnum};

#[derive(Debug, Parser)]
#[command(version)]
struct Cli {
    /// The pattern matching algorithm.
    #[arg(short, value_enum, default_value_t = Algorithm::ShiftAnd)]
    algorithm: Algorithm,

    /// Whether the program should show statistics.
    #[arg(long)]
    stats: bool,

    /// The file to perform the search.
    ///
    /// If not passed, will read from the stdin.
    #[arg(short)]
    input: Option<PathBuf>,

    /// The pattern.
    pattern: String,
}

fn file_or_stdin(maybe_path: Option<&PathBuf>) -> Result<Box<dyn Read>> {
    Ok(match maybe_path {
        Some(path) => Box::new(File::open(path)?),
        None => Box::new(io::stdin()),
    })
}

#[derive(Copy, Clone, Debug, ValueEnum)]
enum Algorithm {
    RabinKarp,
    ShiftAnd,
}

impl Algorithm {
    fn scoped<F, R>(self, text: &[u8], pattern: &[u8], f: F) -> R
    where
        F: FnOnce(&mut dyn Search) -> R,
    {
        macro_rules! select_algorithm {
            ( match $this:ident { $( $branch:pat => $alg:ty, )+ } ) => {
                match $this {
                    $(
                        $branch => {
                            let mut alg: $alg = <$alg>::new(text, pattern);
                            f(&mut alg)
                        }
                    )+
                }
            };
        }
        select_algorithm!(match self {
            Algorithm::RabinKarp => rabin_karp::RabinKarp,
            Algorithm::ShiftAnd => shift_and::ShiftAnd,
        })
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let mut src = file_or_stdin(cli.input.as_ref())?;

    let mut contents = Vec::<u8>::new();
    src.read_to_end(&mut contents)?;

    let pattern = cli.pattern.as_bytes();

    measure_time(cli.stats, || {
        cli.algorithm.scoped(&contents, pattern, |alg| {
            let mut found = false;
            while let Some(i) = alg.search() {
                found |= true;
                println!("{i}");
            }
            if !found {
                println!("no matches");
            }
        });
    });

    Ok(())
}

fn measure_time<F, R>(report: bool, f: F) -> R
where
    F: FnOnce() -> R,
{
    let start = Instant::now();
    let ret = f();
    let elapsed = start.elapsed().as_micros();

    if report {
        println!("(done in {elapsed} μs)");
    }

    ret
}
