extern crate url;

use regex::Regex;
use std::error::Error;

use crate::regexp::regex_core::CoreRegExp;

pub struct UrlMisc;

impl UrlMisc {

    pub fn extract_url(url: &str) -> String {
        let re = Regex::new(CoreRegExp::EXTRACT_URL).unwrap();

        if let Some(capture) = re.find(url) {
            capture.as_str().to_string()
        } else {
            String::new()
        }
    }

    pub fn escape_quotes(url: &str) -> String {
        url.replace("\"", "%22")
    }

    pub fn get_last_part(url: &str) -> String {
        let parts: Vec<&str> = url.split('/').collect();

        if let Some(last_part) = parts.last() {
            last_part.to_string()
        } else {
            String::new()
        }
    }

    pub async fn check_url_status(url: &str) -> Result<(), Box<dyn Error>> {
        let response = reqwest::get(url).await?;
    
        if response.status().as_u16() == 200 {
            Ok(())
        } else {
            Err(
                format!(
                    "Received a non-200 status: {}", 
                    response.status()
                ).into()
            )
        }
    }

}
