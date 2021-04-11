use std::str;

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
            "css" => Extension::Css,
            "csv" => Extension::Csv,
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
            "sh" => Extension::Sh,
            "svg" => Extension::Svg,
            "tar" => Extension::Tar,
            "tiff" | "tif" => Extension::Tiff,
            "ttf" => Extension::Ttf,
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
            _ => return None,
        })
    }

    pub fn mime(&self) -> Mime {
        match self {
            Extension::Css => Mime::TextCss,
            Extension::Csv => Mime::TextCsv,
            Extension::Epub => Mime::ApplicationEpub,
            Extension::Gif => Mime::ImageGif,
            Extension::Html => Mime::TextHtml,
            Extension::Ico => Mime::ImageIco,
            Extension::Ics => Mime::TextCalendar,
            Extension::Js => Mime::ApplicationJavascript,
            Extension::Jpeg => Mime::ImageJpeg,
            Extension::Jsonld => Mime::ApplicationLdJson,
            Extension::Json => Mime::ApplicationJson,
            Extension::Md => Mime::TextMarkdown,
            Extension::Mp3 => Mime::AudioMpeg,
            Extension::Mp4 => Mime::VideoMp4,
            Extension::Mpeg => Mime::VideoMpeg,
            Extension::Oga => Mime::AudioOgg,
            Extension::Ogv => Mime::VideoOgg,
            Extension::Opus => Mime::AudioOpus,
            Extension::Otf => Mime::FontOtf,
            Extension::Pdf => Mime::ApplicationPdf,
            Extension::Php => Mime::ApplicationPhp,
            Extension::Png => Mime::ImagePng,
            Extension::Pl => Mime::ApplicationPerl,
            Extension::Sh => Mime::AppicationSh,
            Extension::Svg => Mime::ImageSvg,
            Extension::Tar => Mime::ApplicationTar,
            Extension::Tiff => Mime::ImageTiff,
            Extension::Ttf => Mime::FontTtf,
            Extension::Wav => Mime::AudioWav,
            Extension::Weba => Mime::AudioWebm,
            Extension::Webm => Mime::VideoWebm,
            Extension::Webp => Mime::ImageWebp,
            Extension::Woff => Mime::FontWoff,
            Extension::Woff2 => Mime::FontWoff2,
            Extension::Xhtml => Mime::ApplicationXhtml,
            Extension::Xls => Mime::ApplicationVndMsExcel,
            Extension::Xlsx => Mime::ApplicationOpenSheet,
            Extension::Xml => Mime::TextXml,
            Extension::Zip => Mime::ApplicationZip,
            Extension::X7zip => Mime::Application7z,
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
            Mime::TextPlain
        } else {
            Mime::OctetStream
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
        assert_eq!(
            Some(Mime::TextPlain),
            Extension::new("rs").map(|e| e.mime())
        );
    }

    #[test]
    fn test_invalid() {
        assert!(Extension::new("hello!").is_none());
    }
}
