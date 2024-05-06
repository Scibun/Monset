pub struct RegexRules;

impl RegexRules {
    
    pub const EXTRACT_URL: &'static str = r"(?P<url>https?://[^\s]+)";
    pub const EXTRACT_PDF_NAME: &'static str = r"/([^/?]+)(?:\?.*)?$";
    pub const VALIDATE_EMAIL: &'static str = r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$";

}
