pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// USB error
    #[error("USB error: {0:?}")]
    USB(#[from] rusb::Error),

    /// Other error
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
