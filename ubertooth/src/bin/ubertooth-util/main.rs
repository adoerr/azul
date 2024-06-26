use anyhow::Result;
use argh::FromArgs;

#[derive(FromArgs)]
/// ubertooth-util - command line utility for Ubertooth One
struct Cli {
    /// count number of devices
    #[argh(switch, short = 'c')]
    count: bool,
    /// info about the firmware
    #[argh(switch, short = 'i')]
    info: bool,
    /// info about the HW board and microcontroller
    #[argh(switch, short = 'b')]
    board: bool,
    /// enable Xmas lights
    #[argh(switch, short = 'x')]
    xmas: bool,
}

fn main() -> Result<()> {
    let cli = argh::from_env::<Cli>();

    if cli.count {
        println!("{}", ubertooth::count()?);
    }

    let ubertooth = ubertooth::Ubertooth::start()?;

    if cli.info {
        println!("{}", ubertooth.info()?);
    } else if cli.board {
        println!("{}", ubertooth.board()?);
    } else if cli.xmas {
        ubertooth.xmas_lights()?;
    }

    Ok(())
}
