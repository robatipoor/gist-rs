use super::errors::*;
use super::token::TOKEN;
use super::url::URL;
use reqwest::{Client, Response};
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::{Read, Write};
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResponseListGist {
    pub id: String,
    pub url: String,
    pub description: Option<String>,
    pub files: HashMap<String, FileGist>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileGist {
    pub filename: String,
    pub r#type: Option<String>,
    pub language: Option<String>,
    pub raw_url: String,
    pub size: u32,
}

pub fn path_gist_file() -> Option<PathBuf> {
    dirs::home_dir().map(|p| p.join(".list-gist"))
}

pub fn read_list_gists(path: &Path) -> Result<Vec<ResponseListGist>> {
    let mut file: File = File::open(path).chain_err(|| "gist file not found")?;
    let mut out = String::new();
    file.read_to_string(&mut out)
        .chain_err(|| "can't read from gist file")?;
    let gists: Vec<ResponseListGist> =
        serde_json::from_str(&out).chain_err(|| "can't read from gist file")?;
    return Ok(gists);
}

pub fn write_file(path: &Path, s: String) -> Result<()> {
    let mut file: File = File::create(path).chain_err(|| "can't create file")?;
    file.write_all(s.as_bytes())
        .chain_err(|| "can't write file")?;
    Ok(())
}

pub fn read_file(path: &Path) -> Result<String> {
    let mut file: File = File::open(path).chain_err(|| "can't open file")?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)
        .chain_err(|| "can't read file")?;
    Ok(buf)
}

pub fn get_name_file(path: &Path) -> Result<String>{
    if path.is_file(){
        Ok(path.file_name().unwrap().to_str().unwrap().to_owned())
    }else{
        Err(Error::from("invalid file path"))
    }
}

pub fn sync_list() -> Result<String> {
    let mut resp: Response = Client::new()
        .get(URL)
        .bearer_auth(TOKEN.clone())
        .send()
        .chain_err(|| "can't get list")?;
    if resp.status().is_success() {
        let list_gist: Vec<ResponseListGist> = resp.json().chain_err(|| "can't read gist list")?;
        let str_list = serde_json::to_string(&list_gist).chain_err(|| "can't get list")?;
        return Ok(str_list);
    }
    return Err(Error::from("unsucess get list gist"));
}

pub fn print_list(gists: Vec<ResponseListGist>, verbose: bool) -> Result<()> {
    for gist in gists {
        println!(
            "{} {}",
            gist.description.unwrap_or("None".to_owned()),
            gist.id
        );
        if verbose {
            for (_, v) in gist.files {
                println!("{}", v.raw_url);
            }
        }
    }
    Ok(())
}
