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

enum Header {
    Allow,
    ContentLength,
    ContentType,
    Server,
}

impl Header {
    fn name(&self) -> &[u8] {
        match self {
            Self::Allow => b"Allow",
            Self::ContentLength => b"Content-Length",
            Self::ContentType => b"Content-Type",
            Self::Server => b"Server",
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
        Self::header(buf, Header::Server, Self::SERVER.as_bytes())?;

        if let Status::MethodNotAllowed { allow } = self.status {
            if !allow.is_empty() {
                Self::header_with(buf, Header::Allow, |buf| {
                    let total = allow.len() - 1;

                    for (idx, method) in allow.iter().enumerate() {
                        buf.write_all(method)?;

                        if idx < total {
                            buf.write_all(b", ")?;
                        }
                    }

                    Ok(())
                })?;
            }
        }

        if let Some(mime) = self.extension.map(|e| Extension::new(e).mime()) {
            Self::header(buf, Header::ContentType, mime.as_bytes())?;
        }

        Self::header(
            buf,
            Header::ContentLength,
            self.content.len().to_string().as_bytes(),
        )?;
        buf.write_all(b"\r\n")?;

        buf.write_all(&self.content)
    }

    fn header(buf: &mut dyn Write, header: Header, value: &[u8]) -> Result<(), IoError> {
        Self::header_with(buf, header, |buf| buf.write_all(value))
    }

    fn header_with(
        buf: &mut dyn Write,
        header: Header,
        f: impl FnOnce(&mut dyn Write) -> Result<(), IoError>,
    ) -> Result<(), IoError> {
        buf.write_all(header.name())?;
        buf.write_all(b": ")?;
        f(buf)?;

        buf.write_all(b"\r\n")
    }
}

#[cfg(test)]
mod tests {
    use super::{Header, PreparedResponse, Response};
    use crate::extension::Extension;
    use std::error::Error;

    #[test]
    fn test_header_names() {
        assert_eq!(b"Allow", Header::Allow.name());
        assert_eq!(b"Content-Length", Header::ContentLength.name());
        assert_eq!(b"Content-Type", Header::ContentType.name());
        assert_eq!(b"Server", Header::Server.name());
    }

    #[test]
    fn test_response_header() -> Result<(), Box<dyn Error + Send + Sync>> {
        let mut buf = Vec::new();
        PreparedResponse::header(&mut buf, Header::ContentType, Extension::Json.mime().as_bytes())?;
        assert_eq!(b"Content-Type: application/json\r\n".as_ref(), buf);

        Ok(())
    }

    #[test]
    fn test_response_header_with() -> Result<(), Box<dyn Error + Send + Sync>> {
        let mut buf = Vec::new();
        PreparedResponse::header_with(&mut buf, Header::Allow, |buf| {
            buf.write_all(b"GET")?;

            buf.write_all(b", POST")
        })?;
        assert_eq!(b"Allow: GET, POST\r\n".as_ref(), buf);

        Ok(())
    }

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
