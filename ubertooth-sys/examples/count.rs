use anyhow::Result;
use ubertooth_sys as sys;

fn main() -> Result<()> {
    let cnt = unsafe { sys::ubertooth_count() };
    println!("count: {}", cnt);

    Ok(())
}
