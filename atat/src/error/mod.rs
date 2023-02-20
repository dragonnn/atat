mod cme_error;
mod cms_error;
mod connection_error;

pub use cme_error::CmeError;
pub use cms_error::CmsError;
pub use connection_error::ConnectionError;

/// Errors returned used internally within the crate
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InternalError<'a> {
    /// Serial read error
    Read,
    /// Serial write error
    Write,
    /// Timed out while waiting for a response
    Timeout,
    /// Invalid response from module
    InvalidResponse,
    /// Command was aborted
    Aborted,
    /// Buffer overflow
    Overflow,
    /// Failed to parse received response
    Parse,
    /// Error response containing any error message
    Error,
    /// GSM Equipment related error
    CmeError(CmeError),
    /// GSM Network related error
    CmsError(CmsError),
    /// Connection Error
    ConnectionError(ConnectionError),
    /// Custom error match
    Custom(&'a [u8]),
}

impl<'a> From<&'a [u8]> for InternalError<'a> {
    fn from(b: &'a [u8]) -> Self {
        match &b[0] {
            0x00 => InternalError::Read,
            0x01 => InternalError::Write,
            0x02 => InternalError::Timeout,
            0x03 => InternalError::InvalidResponse,
            0x04 => InternalError::Aborted,
            0x05 => InternalError::Overflow,
            // 0x06 => InternalError::Parse,
            0x07 => InternalError::Error,
            0x08 => InternalError::CmeError(u16::from_le_bytes(b[1..3].try_into().unwrap()).into()),
            0x09 => InternalError::CmsError(u16::from_le_bytes(b[1..3].try_into().unwrap()).into()),
            0x10 if !b.is_empty() => InternalError::ConnectionError(b[1].into()),
            0x11 if !b.is_empty() => InternalError::Custom(&b[1..]),
            _ => InternalError::Parse,
        }
    }
}

#[cfg(feature = "defmt")]
impl<'a> defmt::Format for InternalError<'a> {
    fn format(&self, f: defmt::Formatter) {
        match self {
            InternalError::Read => defmt::write!(f, "InternalError::Read"),
            InternalError::Write => defmt::write!(f, "InternalError::Write"),
            InternalError::Timeout => defmt::write!(f, "InternalError::Timeout"),
            InternalError::InvalidResponse => defmt::write!(f, "InternalError::InvalidResponse"),
            InternalError::Aborted => defmt::write!(f, "InternalError::Aborted"),
            InternalError::Overflow => defmt::write!(f, "InternalError::Overflow"),
            InternalError::Parse => defmt::write!(f, "InternalError::Parse"),
            InternalError::Error => defmt::write!(f, "InternalError::Error"),
            InternalError::CmeError(e) => defmt::write!(f, "InternalError::CmeError({:?})", e),
            InternalError::CmsError(e) => defmt::write!(f, "InternalError::CmsError({:?})", e),
            InternalError::ConnectionError(e) => {
                defmt::write!(f, "InternalError::ConnectionError({:?})", e)
            }
            InternalError::Custom(e) => {
                defmt::write!(f, "InternalError::Custom({=[u8]:a})", &e)
            }
        }
    }
}

pub enum Response<'a> {
    Result(Result<&'a [u8], InternalError<'a>>),
    Prompt(u8),
}

/// Errors returned by the crate
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {
    /// Serial read error
    Read,
    /// Serial write error
    Write,
    /// Timed out while waiting for a response
    Timeout,
    /// Invalid response from module
    InvalidResponse,
    /// Command was aborted
    Aborted,
    /// Buffer overflow
    Overflow,
    /// Failed to parse received response
    Parse,
    /// Generic error response without any error message
    Error,
    /// GSM Equipment related error
    CmeError(CmeError),
    /// GSM Network related error
    CmsError(CmsError),
    /// Connection Error
    ConnectionError(ConnectionError),
    /// Error response containing any error message
    Custom,
    #[cfg(feature = "custom-error-messages")]
    CustomMessage(heapless::Vec<u8, 64>),
}

impl<'a> From<InternalError<'a>> for Error {
    fn from(ie: InternalError) -> Self {
        match ie {
            InternalError::Read => Self::Read,
            InternalError::Write => Self::Write,
            InternalError::Timeout => Self::Timeout,
            InternalError::InvalidResponse => Self::InvalidResponse,
            InternalError::Aborted => Self::Aborted,
            InternalError::Overflow => Self::Overflow,
            InternalError::Parse => Self::Parse,
            InternalError::Error => Self::Error,
            InternalError::CmeError(e) => Self::CmeError(e),
            InternalError::CmsError(e) => Self::CmsError(e),
            InternalError::ConnectionError(e) => Self::ConnectionError(e),
            #[cfg(feature = "custom-error-messages")]
            InternalError::Custom(e) => Self::CustomMessage(
                heapless::Vec::from_slice(&e[..core::cmp::min(e.len(), 64)]).unwrap_or_default(),
            ),
            #[cfg(not(feature = "custom-error-messages"))]
            InternalError::Custom(_) => Self::Custom,
        }
    }
}
