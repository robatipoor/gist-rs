use crate::errors::*;
use crate::token::*;
use reqwest::{Client,Response};

pub fn delete_gist(url: &str) -> Result<()> {
    let c: Response = Client::new()
        .delete(url)
        .bearer_auth(TOKEN.clone())
        .send()
        .chain_err(|| "can't delete gist")?;

    if c.status().is_success() {
        return Ok(());
    } else {
        return Err(Error::from("delete unsucess"));
    }
}
