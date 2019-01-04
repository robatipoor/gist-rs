use super::post::GistPost;
use super::token::TOKEN;
use crate::errors::*;
use reqwest::{Client, Response};
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GistUpdate {
        pub description: Option<String>,
        pub files: HashMap<String, FileUpdate>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileUpdate {
        pub content: String,
        pub filename: Option<String>,
}

impl GistUpdate {
        pub fn new(cont: String, desc: String, old_name: String, new_name: Option<String>) -> Self {
                let mut hm: HashMap<String, FileUpdate> = HashMap::new();
                hm.insert(
                        old_name,
                        FileUpdate {
                                content: cont,
                                filename: new_name,
                        },
                );
                GistUpdate {
                        description: Some(desc),
                        files: hm,
                }
        }
        pub fn update(&self, url: &str) -> Result<GistPost> {
                let mut resp: Response = Client::new()
                        .patch(url)
                        .bearer_auth(TOKEN.clone())
                        .json(self)
                        .send()
                        .chain_err(|| "update gist faild")?;
                let gist_spot: GistPost = resp.json().chain_err(|| "convert to GistPost faild")?;
                Ok(gist_spot)
        }
}
