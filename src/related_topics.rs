use std::usize;

use crate::client::*;
use crate::utils;
use chrono::prelude::*;
use reqwest::Url;
use serde_json::{Result, Value};

// Correpond to Multiline request => Google trend interest curve
#[derive(Debug)]
pub struct RelatedTopics {
    pub end_date: Date<Utc>,   // Default : Today
    pub start_date: Date<Utc>, // Default : Today
    token: String,
    request: Value,
    client: Client,
}

impl RelatedTopics {
    const RELATED_SEARCH_ENDPOINT: &'static str =
        "https://trends.google.com/trends/api/widgetdata/relatedsearches";
    
        const BAD_CHARACTER: usize = 5;

    pub fn new(client: Client) -> RelatedTopics {
        let end_date = Utc::now().date();
        let start_date = Utc::now().with_year(end_date.year() - 1).unwrap().date();

        let widgets: Value = serde_json::from_str(client.response.as_str()).unwrap();

        let request = widgets["widgets"][2]["request"].clone();
        let token = widgets["widgets"][2]["token"].to_string().replace("\"", "");

        RelatedTopics {
            end_date,
            start_date,
            request,
            token,
            client,
        }
    }

    pub fn get(&self) -> Result<Value> {
        let url = Url::parse(Self::RELATED_SEARCH_ENDPOINT).unwrap();

        let resp = self.client.build_request(
            url.clone(),
            self.request.to_string(),
            self.token.clone(),
        ).send();
        
        let resp = match resp {
            Ok(resp) => resp,
            Err(error) => panic!("Can't get client response: {:?}", error),
        };

        let body = resp.text().unwrap();
        let clean_response = utils::sanitize_response(&body, Self::BAD_CHARACTER);
        serde_json::from_str(clean_response)
    }
}
