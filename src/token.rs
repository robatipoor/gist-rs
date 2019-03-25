use crate::constants::*;
use crate::errors::*;
use crate::utils;
use lazy_static::lazy_static;
use std::fs::File;
use std::io::prelude::*;

lazy_static! {
    pub static ref TOKEN: String = Token::read().unwrap().get();
}

pub struct Token(String);

impl Token {
    fn new(token: String) -> Token {
        return Token(token);
    }

    pub fn get(&self) -> String {
        return self.0.clone();
    }

    pub fn read() -> Result<Self> {
        let path_file = utils::path_file_in_home(TOKEN_FILE_NAME).unwrap();
        let mut file = File::open(path_file.as_path())
            .chain_err(|| format!("failed open file {}", path_file.to_str().unwrap()))?;
        let mut token = String::new();
        file.read_to_string(&mut token)
            .chain_err(|| format!("failed read file {}", path_file.to_str().unwrap()))?;
        return Ok(Token::new(token));
    }

    pub fn write(token: String) -> Result<()> {
        let path_file = utils::path_file_in_home(TOKEN_FILE_NAME).unwrap();
        let mut file = File::create(path_file.as_path())
            .chain_err(|| format!("failed create file {}", path_file.to_str().unwrap()))?;
        file.write(token.as_bytes())
            .chain_err(|| format!("failed write file {}", path_file.to_str().unwrap()))?;
        Ok(())
    }
}
