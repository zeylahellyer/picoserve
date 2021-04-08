pub enum Extension {
    Css,
    Csv,
    Epub,
    Gif,
    Html,
    Ico,
    Ics,
    Js,
    Jpeg,
    Jsonld,
    Json,
    Md,
    Mp3,
    Mp4,
    Mpeg,
    Oga,
    Ogv,
    Opus,
    Otf,
    OtherBinary,
    OtherText,
    Pdf,
    Php,
    Png,
    Pl,
    Sh,
    Svg,
    Tar,
    Tiff,
    Ttf,
    Wav,
    Weba,
    Webm,
    Webp,
    Woff2,
    Woff,
    Xhtml,
    Xlsx,
    Xls,
    Xml,
    Zip,
    X7zip,
}

impl Extension {
    /// Retrieve a MIME type from a common list of extensions.
    ///
    /// If an extension isn't in the predetermined list, then the type
    /// `application/octet-stream` is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use piserve::extension::Extension;
    ///
    /// assert_eq!("text/css", Extension::new("css").name());
    /// ```
    pub fn new(extension: &str) -> Self {
        match extension {
            "css" => Extension::Css,
            "csv" => Extension::Csv,
            "cs" => Extension::OtherText,
            "c" => Extension::OtherText,
            "epub" => Extension::Epub,
            "gif" => Extension::Gif,
            "html" | "htm" => Extension::Html,
            "ico" => Extension::Ico,
            "ics" => Extension::Ics,
            "js" => Extension::Js,
            "jpeg" | "jpg" => Extension::Jpeg,
            "jsonld" => Extension::Jsonld,
            "json" => Extension::Json,
            "md" => Extension::Md,
            "mp3" => Extension::Mp3,
            "mp4" => Extension::Mp4,
            "mpeg" => Extension::Mpeg,
            "oga" => Extension::Oga,
            "ogv" => Extension::Ogv,
            "opus" => Extension::Opus,
            "otf" => Extension::Otf,
            "pdf" => Extension::Pdf,
            "php" => Extension::Php,
            "png" => Extension::Png,
            "pl" => Extension::Pl,
            "py" => Extension::OtherText,
            "rb" => Extension::OtherText,
            "rs" => Extension::OtherText,
            "sh" => Extension::Sh,
            "svg" => Extension::Svg,
            "tar" => Extension::Tar,
            "tiff" | "tif" => Extension::Tiff,
            "toml" => Extension::OtherText,
            "ts" => Extension::OtherText,
            "ttf" => Extension::Ttf,
            "txt" => Extension::OtherText,
            "wav" => Extension::Wav,
            "weba" => Extension::Weba,
            "webm" => Extension::Webm,
            "webp" => Extension::Webp,
            "woff" => Extension::Woff,
            "woff2" => Extension::Woff2,
            "xhtml" => Extension::Xhtml,
            "xls" => Extension::Xls,
            "xlsx" => Extension::Xlsx,
            "xml" => Extension::Xml,
            "zip" => Extension::Zip,
            "7z" => Extension::X7zip,
            _ => Extension::OtherBinary,
        }
    }

    pub fn mime(&self) -> &'static str {
        match self {
            Extension::Css => "text/css",
            Extension::Csv => "text/csv",
            Extension::Epub => "application/epub+zip",
            Extension::Gif => "image/gif",
            Extension::Html => "text/html",
            Extension::Ico => "image/vnd.microsoft.icon",
            Extension::Ics => "text/calendar",
            Extension::Js => "application/javascript",
            Extension::Jpeg => "image/jpeg",
            Extension::Jsonld => "application/ld+json",
            Extension::Json => "application/json",
            Extension::Md => "text/markdown",
            Extension::Mp3 => "audio/mpeg",
            Extension::Mp4 => "video/mp4",
            Extension::Mpeg => "video/mpeg",
            Extension::Oga => "audio/ogg",
            Extension::Ogv => "video/ogg",
            Extension::Opus => "audio/opus",
            Extension::Otf => "font/otf",
            Extension::Pdf => "application/pdf",
            Extension::Php => "application/x-httpd-php",
            Extension::Png => "image/png",
            Extension::Pl => "application/x-perl",
            Extension::Sh => "application/x-sh",
            Extension::Svg => "image/svg+xml",
            Extension::Tar => "application/x-tar",
            Extension::Tiff => "image/tiff",
            Extension::Ttf => "font/ttf",
            Extension::Wav => "audio/wav",
            Extension::Weba => "audio/webm",
            Extension::Webm => "video/webm",
            Extension::Webp => "image/webp",
            Extension::Woff => "font/woff",
            Extension::Woff2 => "font/woff2",
            Extension::Xhtml => "application/xhtml+xml",
            Extension::Xls => "application/vnd.ms-excel",
            Extension::Xlsx => "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
            Extension::Xml => "text/xml",
            Extension::Zip => "application/zip",
            Extension::X7zip => "application/x-7z-compressed",
            Extension::OtherBinary => "application/octet-stream",
            Extension::OtherText => "text/plain",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Extension;

    #[test]
    fn test_valid() {
        assert_eq!("text/html", Extension::new("html").mime());
        assert_eq!("text/plain", Extension::new("rs").mime());
    }

    #[test]
    fn test_invalid() {
        assert_eq!("application/octet-stream", Extension::new("hello!").mime());
    }
}
