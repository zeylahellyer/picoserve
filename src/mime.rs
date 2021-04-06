/// Retrieve a MIME type from a common list of extensions.
///
/// If an extension isn't in the predetermined list, then the type
/// `application/octet-stream` is returned.
///
/// # Examples
///
/// ```
/// assert_eq!("text/css", picoserve::mime::from_ext("css"));
/// ```
pub fn from_ext(extension: &str) -> &str {
    match extension {
        "css" => "text/css",
        "csv" => "text/csv",
        "cs" => "text/plain",
        "c" => "text/plain",
        "epub" => "application/epub+zip",
        "gif" => "image/gif",
        "html" | "htm" => "text/html",
        "ico" => "image/vnd.microsoft.icon",
        "ics" => "text/calendar",
        "js" => "application/javascript",
        "jpeg" | "jpg" => "image/jpeg",
        "json" => "application/json",
        "jsonld" => "application/ld+json",
        "md" => "text/markdown",
        "mp3" => "audio/mpeg",
        "mp4" => "video/mp4",
        "mpeg" => "video/mpeg",
        "oga" => "audio/ogg",
        "ogv" => "video/ogg",
        "opus" => "audio/opus",
        "otf" => "font/otf",
        "pdf" => "application/pdf",
        "php" => "application/x-httpd-php",
        "png" => "image/png",
        "pl" => "application/x-perl",
        "py" => "text/plain",
        "rb" => "text/plain",
        "rs" => "text/plain",
        "sh" => "application/x-sh",
        "svg" => "image/svg+xml",
        "tar" => "application/x-tar",
        "tiff" | "tif" => "image/tiff",
        "toml" => "text/plain",
        "ts" => "text/plain",
        "ttf" => "font/ttf",
        "txt" => "text/plain",
        "wav" => "audio/wav",
        "weba" => "audio/webm",
        "webm" => "video/webm",
        "webp" => "image/webp",
        "woff" => "font/woff",
        "woff2" => "font/woff2",
        "xhtml" => "application/xhtml+xml",
        "xls" => "application/vnd.ms-excel",
        "xlsx" => "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
        "xml" => "text/xml",
        "zip" => "application/zip",
        "7z" => "application/x-7z-compressed",
        _ => "application/octet-stream",
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_valid() {
        assert_eq!("text/html", super::from_ext("html"));
        assert_eq!("text/plain", super::from_ext("rs"));
    }

    #[test]
    fn test_invalid() {
        assert_eq!("application/octet-stream", super::from_ext("hello!"));
    }
}
