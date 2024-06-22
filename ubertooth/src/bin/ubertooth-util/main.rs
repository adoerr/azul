use anyhow::Result;
use argh::FromArgs;

#[derive(FromArgs)]
/// ubertooth-util - command line utility for Ubertooth One
struct Cli {
    /// count number of devices
    #[argh(switch, short = 'c')]
    count: bool,
}

fn main() -> Result<()> {
    let cli = argh::from_env::<Cli>();

    if cli.count {
        println!("{}", ubertooth::count()?);
    }

    Ok(())
}
