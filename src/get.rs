use crate::token::*;
use crate::errors::*;
use reqwest::{Client, Response};
use std::io::Read;

pub fn get_gist(url: String) -> Result<String> {
    let mut resp: Response = Client::new()
        .get(&url)
        .bearer_auth(TOKEN.clone())
        .send()
        .chain_err(|| "can't get list")?;
    if resp.status().is_success() {
        let mut buf = String::new();
        resp.read_to_string(&mut buf).unwrap();
        return Ok(buf);
    }
    Err(Error::from("get unsuccess "))
}
