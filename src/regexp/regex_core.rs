pub struct CoreRegExp;

impl CoreRegExp {
     
    pub const EXTRACT_URL: &'static str = r"(?P<url>https?://[^\s]+)";
    pub const VALIDATE_TAGS: &'static str = r".*\*{1,2}.*|.*_.*|.*<\s*[a-zA-Z]+[^>]*>.*";

    pub const RENDER_EXTRA_GIST: &'static str = r#"\[!extra gist data=['"](.*?)['"]\]"#;
    pub const RENDER_EXTRA_QRCODE: &'static str = r#"\[!extra qrcode data=['"](.*?)['"], size=(\d+)\]"#;

    pub const GET_CHECKSUM: &'static str = r"(?i)\b([a-f0-9]{64})\b\s+(.+)";

    pub const CLEAN_LINE: &'static str = r"\s*\{\s*\}\s*";

}
