#![allow(dead_code)]

#[repr(u8)]
#[allow(clippy::upper_case_acronyms)]
/// Ubertooth USB commands
pub(crate) enum Commands {
    PING = 0,
    GET_USRLED = 3,
    SET_USRLED = 4,
}

#[repr(u16)]
#[allow(clippy::upper_case_acronyms)]
pub enum Led {
    OFF = 0,
    ON = 1,
}
