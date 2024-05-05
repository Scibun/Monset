extern crate colored;

use colored::*;

extern crate reqwest;

use std::error::Error;

use serde::Deserialize;

use crate::cmd::download::Download;

use crate::configs::apis_uri::ApisUri;

#[derive(Debug, Deserialize)]
struct Item {
    url: String,
    encrypted: bool,
}

#[derive(Debug, Deserialize)]
struct Response {
    #[serde(default)]
    total: Option<u32>,

    #[serde(default)]
    list: Vec<Item>,

    #[serde(default)]
    message: String,

    success: Option<bool>
}

pub struct Scrape;

impl Scrape {

    async fn fetch_items(url: &str) -> Result<Response, Box<dyn std::error::Error>> {
        let url_with_param = ApisUri::PAIMON_SCRAPE_API_REQUEST.to_string() + url;
        let response = reqwest::get(&url_with_param).await?;
        let body = response.bytes().await?;
        
        let data = serde_json::from_slice(&body)?;
        Ok(data)
    }

    pub async fn get(scrape: bool, url: &str, no_ignore: bool, no_comments: bool, kindle: Option<String>) -> Result<(), Box<dyn Error>> {
        if scrape {
            match Self::fetch_items(url).await {
                Ok(response) => {
                    if let Some(success) = response.success {
                        if !success {
                            eprintln!("Error: {}", response.message.red());
                            return Ok(())
                        }
                    }
    
                    if let Some(total) = response.total {
                        if total > 0 {
                            if !response.list.is_empty() {
                                for item in &response.list {
                                    if !item.encrypted {
                                        Download::download_file(
                                            &item.url, 
                                            "",
                                            no_ignore, 
                                            no_comments, 
                                            kindle.clone()
                                        ).await?;
                                    }
                                }
                            }
                        } else {
                            println!("{}", response.message.red());
                        }
                    }
                }

                Err(e) => eprintln!("Error: {}", e.to_string().red())
            }
        }
    
        Ok(())
    }
    
}
