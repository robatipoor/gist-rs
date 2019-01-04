use crate::errors::*;
use crate::list_gist::*;
use std::path::Path;

pub const URL: &'static str = "https://api.github.com/gists";

pub fn search_url(path: &Path, id: &str) -> Result<String> {
    if id.len() < 5 {
        return Err(Error::from("id invalid"));
    }
    let gists = read_list_gists(path).chain_err(|| "can't read list gist")?;
    for gist in gists {
        if gist.id.starts_with(id) {
            return Ok(gist.url);
        }
    }
    return Err(Error::from("gist not exist"));
}

pub fn search_raw_url(path: &Path, id: &str) -> Result<String> {
    if id.len() < 5 {
        return Err(Error::from("id len most be bigger than 5"));
    }
    let gists = read_list_gists(path).chain_err(|| "can't read list gist")?;
    for gist in gists {
        if gist.id.starts_with(id) {
            for (_, v) in gist.files {
                return Ok(v.raw_url);
            }
        }
    }
    return Err(Error::from("gist not exist"));
}
