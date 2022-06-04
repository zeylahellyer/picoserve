use core::str;

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
    pub fn new(extension: &str) -> Option<Self> {
        Some(match extension {
            "css" => Self::Css,
            "csv" => Self::Csv,
            "epub" => Self::Epub,
            "gif" => Self::Gif,
            "html" | "htm" => Self::Html,
            "ico" => Self::Ico,
            "ics" => Self::Ics,
            "js" => Self::Js,
            "jpeg" | "jpg" => Self::Jpeg,
            "jsonld" => Self::Jsonld,
            "json" => Self::Json,
            "md" => Self::Md,
            "mp3" => Self::Mp3,
            "mp4" => Self::Mp4,
            "mpeg" => Self::Mpeg,
            "oga" => Self::Oga,
            "ogv" => Self::Ogv,
            "opus" => Self::Opus,
            "otf" => Self::Otf,
            "pdf" => Self::Pdf,
            "php" => Self::Php,
            "png" => Self::Png,
            "pl" => Self::Pl,
            "sh" => Self::Sh,
            "svg" => Self::Svg,
            "tar" => Self::Tar,
            "tiff" | "tif" => Self::Tiff,
            "ttf" => Self::Ttf,
            "wav" => Self::Wav,
            "weba" => Self::Weba,
            "webm" => Self::Webm,
            "webp" => Self::Webp,
            "woff" => Self::Woff,
            "woff2" => Self::Woff2,
            "xhtml" => Self::Xhtml,
            "xls" => Self::Xls,
            "xlsx" => Self::Xlsx,
            "xml" => Self::Xml,
            "zip" => Self::Zip,
            "7z" => Self::X7zip,
            _ => return None,
        })
    }

    pub fn mime(&self) -> Mime {
        match self {
            Self::Css => Mime::TextCss,
            Self::Csv => Mime::TextCsv,
            Self::Epub => Mime::ApplicationEpub,
            Self::Gif => Mime::ImageGif,
            Self::Html => Mime::TextHtml,
            Self::Ico => Mime::ImageIco,
            Self::Ics => Mime::TextCalendar,
            Self::Js => Mime::ApplicationJavascript,
            Self::Jpeg => Mime::ImageJpeg,
            Self::Jsonld => Mime::ApplicationLdJson,
            Self::Json => Mime::ApplicationJson,
            Self::Md => Mime::TextMarkdown,
            Self::Mp3 => Mime::AudioMpeg,
            Self::Mp4 => Mime::VideoMp4,
            Self::Mpeg => Mime::VideoMpeg,
            Self::Oga => Mime::AudioOgg,
            Self::Ogv => Mime::VideoOgg,
            Self::Opus => Mime::AudioOpus,
            Self::Otf => Mime::FontOtf,
            Self::Pdf => Mime::ApplicationPdf,
            Self::Php => Mime::ApplicationPhp,
            Self::Png => Mime::ImagePng,
            Self::Pl => Mime::ApplicationPerl,
            Self::Sh => Mime::AppicationSh,
            Self::Svg => Mime::ImageSvg,
            Self::Tar => Mime::ApplicationTar,
            Self::Tiff => Mime::ImageTiff,
            Self::Ttf => Mime::FontTtf,
            Self::Wav => Mime::AudioWav,
            Self::Weba => Mime::AudioWebm,
            Self::Webm => Mime::VideoWebm,
            Self::Webp => Mime::ImageWebp,
            Self::Woff => Mime::FontWoff,
            Self::Woff2 => Mime::FontWoff2,
            Self::Xhtml => Mime::ApplicationXhtml,
            Self::Xls => Mime::ApplicationVndMsExcel,
            Self::Xlsx => Mime::ApplicationOpenSheet,
            Self::Xml => Mime::TextXml,
            Self::Zip => Mime::ApplicationZip,
            Self::X7zip => Mime::Application7z,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Mime {
    ApplicationEpub,
    ApplicationJavascript,
    ApplicationLdJson,
    ApplicationJson,
    OctetStream,
    ApplicationPdf,
    ApplicationVndMsExcel,
    ApplicationOpenSheet,
    Application7z,
    ApplicationPhp,
    ApplicationPerl,
    AppicationSh,
    ApplicationTar,
    ApplicationXhtml,
    ApplicationZip,
    AudioMpeg,
    AudioOgg,
    AudioOpus,
    AudioWav,
    AudioWebm,
    FontOtf,
    FontTtf,
    FontWoff,
    FontWoff2,
    ImageGif,
    ImageJpeg,
    ImagePng,
    ImageSvg,
    ImageTiff,
    ImageIco,
    ImageWebp,
    TextCalendar,
    TextCss,
    TextCsv,
    TextHtml,
    TextMarkdown,
    TextPlain,
    TextXml,
    VideoMp4,
    VideoMpeg,
    VideoOgg,
    VideoWebm,
}

impl Mime {
    pub fn from_input(input: &[u8]) -> Self {
        if str::from_utf8(input).is_ok() {
            Self::TextPlain
        } else {
            Self::OctetStream
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::ApplicationEpub => "application/epub+zip",
            Self::ApplicationJavascript => "application/javascript",
            Self::ApplicationLdJson => "application/ld+json",
            Self::ApplicationJson => "application/json",
            Self::OctetStream => "application/octet-stream",
            Self::ApplicationPdf => "application/pdf",
            Self::ApplicationVndMsExcel => "application/vnd.ms-excel",
            Self::ApplicationOpenSheet => {
                "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"
            }
            Self::Application7z => "application/x-7z-compressed",
            Self::ApplicationPhp => "application/x-httpd-php",
            Self::ApplicationPerl => "application/x-perl",
            Self::AppicationSh => "application/x-sh",
            Self::ApplicationTar => "application/x-tar",
            Self::ApplicationXhtml => "application/xhtml+xml",
            Self::ApplicationZip => "application/zip",
            Self::AudioMpeg => "audio/mpeg",
            Self::AudioOgg => "audio/ogg",
            Self::AudioOpus => "audio/opus",
            Self::AudioWav => "audio/wav",
            Self::AudioWebm => "audio/webm",
            Self::FontOtf => "font/otf",
            Self::FontTtf => "font/ttf",
            Self::FontWoff => "font/woff",
            Self::FontWoff2 => "font/woff2",
            Self::ImageGif => "image/gif",
            Self::ImageJpeg => "image/jpeg",
            Self::ImagePng => "image/png",
            Self::ImageSvg => "image/svg+xml",
            Self::ImageTiff => "image/tiff",
            Self::ImageIco => "image/vnd.microsoft.icon",
            Self::ImageWebp => "image/webp",
            Self::TextCalendar => "text/calendar",
            Self::TextCss => "text/css",
            Self::TextCsv => "text/csv",
            Self::TextHtml => "text/html",
            Self::TextMarkdown => "text/markdown",
            Self::TextPlain => "text/plain",
            Self::TextXml => "text/xml",
            Self::VideoMp4 => "video/mp4",
            Self::VideoMpeg => "video/mpeg",
            Self::VideoOgg => "video/ogg",
            Self::VideoWebm => "video/webm",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Extension, Mime};

    #[test]
    fn test_valid() {
        assert_eq!(
            Some(Mime::TextHtml),
            Extension::new("html").map(|e| e.mime())
        );
        assert!(Extension::new("rs").map(|e| e.mime()).is_none());
    }

    #[test]
    fn test_invalid() {
        assert!(Extension::new("hello!").is_none());
    }
}
