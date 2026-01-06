mod rainbow;
mod config;
use config::*;
use std::io::{self, Read, Write};

fn main() -> io::Result<()> {
    let config = match Config::from_args() {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let output = rainbow::rainbow(&input, config);

    let mut stdout = io::stdout().lock();
    stdout.write_all(output.as_bytes())?;
    stdout.flush()?;

    Ok(())
}
