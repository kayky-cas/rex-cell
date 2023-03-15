use std::{collections::HashMap, env, fs, str::FromStr};

use anyhow::anyhow;

// Usage: ./run ./inputs/input1.csv
fn main() -> anyhow::Result<()> {
    let program_name = env::args().nth(0).ok_or(anyhow!("Invalid program name"))?;

    let args: Vec<_> = env::args().skip(1).collect();

    if args.is_empty() {
        return Err(anyhow!(
            "{} needs at least 1 argument to execute.",
            program_name
        ));
    }

    let input_name = dbg!(&args[0]);
    let buffer = dbg!(fs::read_to_string(input_name)?);

    Ok(())
}
