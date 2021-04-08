use crate::mime;
use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
    io::{Error as IoError, Write},
};

#[derive(Debug)]
pub enum WriteError {
    /// Failed to write data to an IO writer.
    Io {
        /// Reason for the error.
        source: IoError,
    },
}

impl Display for WriteError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Io { .. } => f.write_str("failed to write data to writer"),
        }
    }
}

impl Error for WriteError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Io { source } => Some(source),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Status {
    Forbidden,
    InternalServiceError,
    MethodNotAllowed,
    NotFound,
    Ok,
}

impl Status {
    fn name(&self) -> &[u8] {
        match self {
            Self::Forbidden => b"403 FORBIDDEN",
            Self::InternalServiceError => b"500 INTERNAL SERVICE ERROR",
            Self::MethodNotAllowed => b"405 METHOD NOT ALLOWED",
            Self::NotFound => b"404 NOT FOUND",
            Self::Ok => b"200 OK",
        }
    }
}

/// Write a response to a writer.
///
/// Uses a provided status, content, and extension type to write the response.
///
/// Extension is optional and will be mapped to a MIME when provided.
pub fn write(
    buf: &mut dyn Write,
    status: Status,
    content: &[u8],
    extension: Option<&str>,
    allow: Option<&[&[u8]]>,
) -> Result<(), WriteError> {
    write_inner(buf, status, content, extension, allow).map_err(|source| WriteError::Io { source })
}

fn write_inner(
    buf: &mut dyn Write,
    status: Status,
    content: &[u8],
    extension: Option<&str>,
    allow: Option<&[&[u8]]>,
) -> Result<(), IoError> {
    buf.write_all(b"HTTP/1.1 ")?;
    buf.write_all(status.name())?;
    buf.write_all(b"\r\n")?;

    if let Some(allow) = allow {
        if !allow.is_empty() {
            buf.write_all(b"Allow: ")?;

            let total = allow.len() - 1;

            for (idx, method) in allow.iter().enumerate() {
                buf.write_all(method)?;

                if idx < total {
                    buf.write_all(b", ")?;
                }
            }

            buf.write_all(b"\r\n")?;
        }
    }

    if let Some(content_type) = extension.map(mime::from_ext) {
        buf.write_all(b"Content-Type: ")?;
        buf.write_all(content_type.as_bytes())?;
        buf.write_all(b"\r\n")?;
    }

    buf.write_all(b"Content-Length: ")?;
    buf.write_fmt(format_args!("{}", content.len()))?;
    buf.write_all(b"\r\n\r\n")?;

    buf.write_all(&content)
}

#[cfg(test)]
mod tests {
    use super::Status;
    use std::error::Error;

    #[test]
    fn test_ok() -> Result<(), Box<dyn Error + Send + Sync>> {
        let mut buf = Vec::new();

        super::write(&mut buf, Status::Ok, b"test", None, None)?;

        assert_eq!(buf, b"HTTP/1.1 200 OK\r\nContent-Length: 4\r\n\r\ntest");

        Ok(())
    }
}
