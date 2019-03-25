use crate::constants::*;
use crate::errors::*;
use crate::token::*;
use crate::utils;
use colored::*;
use reqwest::{Client, Response};
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct ResponseGist {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "description")]
    pub desc: Option<String>,
    #[serde(rename = "files")]
    pub files: HashMap<String, FileGist>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct FileGist {
    #[serde(rename = "filename")]
    pub name: String,
    #[serde(rename = "type")]
    pub type_file: Option<String>,
    #[serde(rename = "language")]
    pub lang: Option<String>,
    #[serde(rename = "raw_url")]
    pub raw_url: String,
    #[serde(rename = "size")]
    pub size: u32,
}

pub struct ListGist {
    list: Vec<ResponseGist>,
}

impl ListGist {
    fn new(list: Vec<ResponseGist>) -> ListGist {
        return ListGist { list };
    }

    pub fn read() -> Result<ListGist> {
        let path_file = &utils::path_file_in_home(LIST_GIST_FILE_NAME).unwrap();
        let out = utils::read_file(path_file).unwrap();
        let gists: Vec<ResponseGist> = serde_json::from_str(&out)
            .chain_err(|| format!("failed read file {}", path_file.to_str().unwrap()))?;
        return Ok(ListGist::new(gists));
    }

    fn write(&self) -> Result<()> {
        let path_file = utils::path_file_in_home(LIST_GIST_FILE_NAME).unwrap();
        let list_string = serde_json::to_string(&self.list).chain_err(|| "can't get list")?;
        return utils::write_file(path_file, list_string);
    }

    fn get_update_list_gist() -> Result<ListGist> {
        let mut resp: Response = Client::new()
            .get(URL)
            .bearer_auth(&*TOKEN)
            .send()
            .chain_err(|| "failed get list")?;
        if resp.status().is_success() {
            let list_gist: Vec<ResponseGist> = resp.json().chain_err(|| "can't read gist list")?;
            return Ok(ListGist::new(list_gist));
        }
        return Err(Error::from("unsuccessful get list gist"));
    }

    pub fn sync() -> Result<ListGist> {
        let list_gist = ListGist::get_update_list_gist().unwrap();
        list_gist.write().unwrap();        
        Ok(list_gist)
    }

    pub fn _search_url_gist<T: AsRef<str>>(&self, id: T) -> Result<String> {
        if id.as_ref().len() < 5 {
            return Err(Error::from("id invalid"));
        }
        for gist in self.list.clone() {
            if gist.id.starts_with(id.as_ref()) {
                return Ok(gist.url);
            }
        }
        return Err(Error::from("gist file not exist"));
    }

    pub fn search_raw_url_gist<T: AsRef<str>>(&self, id: T) -> Result<String> {
        if id.as_ref().len() < 5 {
            return Err(Error::from("id len most be bigger than 5"));
        }
        for gist in self.list.clone() {
            if gist.id.starts_with(id.as_ref()) {
                for (_, v) in gist.files {
                    return Ok(v.raw_url);
                }
            }
        }
        return Err(Error::from("gist not exist"));
    }

    pub fn get_name_gist_file<T: AsRef<str>>(&self, id: T) -> Result<String> {
        for gist in self.list.clone() {
            if gist.id == id.as_ref() {
                for (_, v) in gist.files {
                    return Ok(v.name);
                }
            }
        }
        return Err(Error::from("id not exist"));
    }

    pub fn print(&self, verbose: bool) -> Result<()> {
        let mut count = 0;
        for gist in self.list.clone() {
            println!(
                "{}) {}",
                count,
                gist.desc.unwrap_or("None Description".to_owned()),
            );
            println!("{}", gist.id.blue().bold());
            if verbose {
                for (_, v) in gist.files {
                    println!("{}", v.raw_url.green());
                    println!("---------------------------------------------------------");
                }
            }
            count += 1;
        }
        Ok(())
    }
}
