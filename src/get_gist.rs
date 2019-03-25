use crate::errors::*;
use crate::token::*;
use reqwest::{Client, Response};
use std::io::Read;

pub fn get_gist_file(url: &str) -> Result<String> {
    let mut resp: Response = Client::new()
        .get(url)
        .bearer_auth(&*TOKEN)
        .send()
        .chain_err(|| format!("failed get gist file {}", url))?;
    if resp.status().is_success() {
        let mut buf = String::new();
        resp.read_to_string(&mut buf).unwrap();
        return Ok(buf);
    }
    Err(Error::from("unsuccessful get gist file"))
}
