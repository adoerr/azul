#![allow(dead_code)]

#[repr(u8)]
#[allow(clippy::upper_case_acronyms)]
/// Ubertooth USB commands
pub(crate) enum Commands {
    PING = 0,
    /// Get user LED state
    GET_USRLED = 3,
    /// Set user LED state
    SET_USRLED = 4,
    /// Get firmware revision number
    GET_REV_NUM = 33,
    /// Get compile information
    GET_COMPILE_INFO = 55,
    /// Enable Christmas lights effect
    XMAS = 73,
}

#[repr(u16)]
#[allow(clippy::upper_case_acronyms)]
pub enum Led {
    OFF = 0,
    ON = 1,
}

#[derive(Debug)]
/// Firmware and compile information
pub struct Info {
    /// Firmware version
    pub version: String,
    /// Compile information
    pub compile: String,
}
