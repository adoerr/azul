use anyhow::Result;
use ubertooth::{Led, Ubertooth};

fn main() -> Result<()> {
    let ut = Ubertooth::init()?;
    println!("version {:?}", ut.version()?);

    println!("user led {:?}", ut.user_led()?);
    ut.set_user_led(Led::ON)?;

    println!("user led {:?}", ut.user_led()?);
    ut.set_user_led(Led::OFF)?;

    Ok(())
}
