use crate::extension::Extension;
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

enum Status<'a> {
    Forbidden,
    InternalServiceError,
    MethodNotAllowed { allow: &'a [&'a [u8]] },
    NotFound,
    Ok,
}

impl Status<'_> {
    fn name(&self) -> &[u8] {
        match self {
            Self::Forbidden => b"403 FORBIDDEN",
            Self::InternalServiceError => b"500 INTERNAL SERVICE ERROR",
            Self::MethodNotAllowed { .. } => b"405 METHOD NOT ALLOWED",
            Self::NotFound => b"404 NOT FOUND",
            Self::Ok => b"200 OK",
        }
    }
}

pub struct Response<'a> {
    content: &'a [u8],
    extension: Option<&'a str>,
}

impl<'a> Response<'a> {
    pub fn new(content: &'a [u8]) -> Self {
        Self {
            content,
            extension: None,
        }
    }

    pub fn extension(mut self, extension: Option<&'a str>) -> Self {
        self.extension = extension;

        self
    }

    pub fn ok(self) -> PreparedResponse<'a> {
        PreparedResponse {
            content: self.content,
            extension: self.extension,
            status: Status::Ok,
        }
    }

    pub fn forbidden(self) -> PreparedResponse<'a> {
        PreparedResponse {
            content: self.content,
            extension: self.extension,
            status: Status::Forbidden,
        }
    }

    pub fn not_found(self) -> PreparedResponse<'a> {
        PreparedResponse {
            content: self.content,
            extension: self.extension,
            status: Status::NotFound,
        }
    }

    pub fn method_not_allowed(self, allow: &'a [&'a [u8]]) -> PreparedResponse<'a> {
        PreparedResponse {
            content: self.content,
            extension: self.extension,
            status: Status::MethodNotAllowed { allow },
        }
    }

    pub fn internal_service_error(self) -> PreparedResponse<'a> {
        PreparedResponse {
            content: self.content,
            extension: self.extension,
            status: Status::InternalServiceError,
        }
    }
}

pub struct PreparedResponse<'a> {
    content: &'a [u8],
    extension: Option<&'a str>,
    status: Status<'a>,
}

impl PreparedResponse<'_> {
    const SERVER: &'static str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

    /// Write a response to a writer.
    ///
    /// Uses a provided status, content, and extension type to write the response.
    ///
    /// Extension is optional and will be mapped to a MIME when provided.
    pub fn write(&self, buf: &mut dyn Write) -> Result<(), WriteError> {
        self.write_inner(buf)
            .map_err(|source| WriteError::Io { source })
    }

    fn write_inner(&self, buf: &mut dyn Write) -> Result<(), IoError> {
        buf.write_all(b"HTTP/1.1 ")?;
        buf.write_all(self.status.name())?;
        buf.write_all(b"\r\n")?;
        buf.write_all(b"Server: ")?;
        buf.write_all(Self::SERVER.as_bytes())?;
        buf.write_all(b"\r\n")?;

        if let Status::MethodNotAllowed { allow } = self.status {
            if !allow.is_empty() {
                buf.write_all(b"Allow: ")?;

                let total = allow.len() - 1;

                for (idx, method) in allow.iter().enumerate() {
                    buf.write_all(method)?;

                    if idx < total {
                        buf.write_all(b", ")?;
                    }
                }
            }

            buf.write_all(b"\r\n")?;
        }

        if let Some(content_type) = self.extension.map(|e| Extension::new(e).mime()) {
            buf.write_all(b"Content-Type: ")?;
            buf.write_all(content_type.as_bytes())?;
            buf.write_all(b"\r\n")?;
        }

        buf.write_all(b"Content-Length: ")?;
        buf.write_fmt(format_args!("{}", self.content.len()))?;
        buf.write_all(b"\r\n\r\n")?;

        buf.write_all(&self.content)
    }
}

#[cfg(test)]
mod tests {
    use super::{PreparedResponse, Response};
    use std::error::Error;

    #[test]
    fn test_ok() -> Result<(), Box<dyn Error + Send + Sync>> {
        let mut buf = Vec::new();
        Response::new(b"test").ok().write(&mut buf)?;

        assert_eq!(
            buf,
            format!(
                "HTTP/1.1 200 OK\r\nServer: {}\r\nContent-Length: 4\r\n\r\ntest",
                PreparedResponse::SERVER
            )
            .into_bytes()
        );

        Ok(())
    }
}
