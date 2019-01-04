use crate::errors::*;
use lazy_static::lazy_static;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

lazy_static! {
    pub static ref TOKEN: String = read_token().expect("cant get token");
}

pub fn path_token() -> Option<PathBuf> {
    dirs::home_dir().map(|p| p.join(".gist-rs"))
}

pub fn read_token() -> Result<String> {
    let mut file = File::open(path_token().unwrap()).chain_err(|| "can't open token file")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .chain_err(|| "can't read token file")?;
    Ok(contents.trim().to_owned())
}

pub fn write_token(token: String) -> Result<()> {
    let mut file = File::create(path_token().unwrap()).chain_err(|| "can't create token file")?;
    file.write(token.as_bytes())
        .chain_err(|| "can't write to token file")?;
    Ok(())
}
