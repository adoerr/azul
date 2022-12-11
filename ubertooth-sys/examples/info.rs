use anyhow::Result;

fn main() -> Result<()> {
    let res = unsafe { ubertooth_sys::ubertooth_count() };

    println!("{res}");

    Ok(())
}