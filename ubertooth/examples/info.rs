use anyhow::Result;
use ubertooth::Ubertooth;

fn main() -> Result<()> {
    let ut = Ubertooth::init()?;
    println!("version {:?}", ut.version()?);

    Ok(())
}
