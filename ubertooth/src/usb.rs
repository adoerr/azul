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
    /// Get device serial number
    GET_SERIAL_NUM = 14,
    /// Get part ID
    GET_PART_ID = 15,
    /// Get firmware revision number
    GET_REV_NUM = 33,
    /// Get board ID
    GET_BOARD_ID = 35,
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

impl std::fmt::Display for Info {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Firmware : {}\nCompile  : {}",
            self.version, self.compile
        )
    }
}
