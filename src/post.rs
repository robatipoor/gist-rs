use crate::errors::*;
use crate::token::TOKEN;
use crate::url::URL;
use reqwest::{Client, Response};
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GistPost {
    pub description: String,
    pub public: bool,
    pub files: HashMap<String, FilePost>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FilePost {
    pub content: String,
}

impl GistPost {
    pub fn new(cont: String, public: bool, desc: String, name: String) -> Self {
        let mut hm: HashMap<String, FilePost> = HashMap::new();
        hm.insert(name, FilePost { content: cont });
        GistPost {
            description: desc,
            public: public,
            files: hm,
        }
    }
    pub fn post(&self) -> Result<GistPost> {
        let mut resp: Response = Client::new()
            .post(URL)
            .bearer_auth(TOKEN.to_owned())
            .json(self)
            .send()
            .chain_err(|| "post gist unsuccess !")?;
        let rs_data: GistPost = resp.json().chain_err(|| "convert to json error")?;
        Ok(rs_data)
    }
}
