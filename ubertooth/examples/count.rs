#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
include!(concat!(env!("OUT_DIR"), "/ubertooth-gen.rs"));

use anyhow::Result;

fn main() -> Result<()> {
    let cnt = unsafe { ubertooth_count() };
    println!("count: {}", cnt);

    Ok(())
}
