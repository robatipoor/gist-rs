use crate::errors::*;
use std::fs::File;
use std::io::{self, prelude::*};
use std::path::{Path, PathBuf};

pub fn read_stdin() -> Option<String> {
    let mut re = String::new();
    return io::stdin().read_to_string(&mut re).ok().map(|_| re);
}

pub fn path_file_in_home(name_file: &str) -> Option<PathBuf> {
    dirs::home_dir().map(|p| p.join(name_file))
}

pub fn read_file<T: AsRef<Path>>(path: T) -> Result<String> {
    let mut file: File = File::open(path).chain_err(|| "gist file not found")?;
    let mut out = String::new();
    file.read_to_string(&mut out)
        .chain_err(|| "can't read from gist file")?;
    return Ok(out);
}

pub fn write_file<T: AsRef<Path>>(path: T, s: String) -> Result<()> {
    let mut file: File = File::create(path).chain_err(|| "can't create file")?;
    file.write_all(s.as_bytes())
        .chain_err(|| "can't write file")?;
    Ok(())
}

pub fn get_name_file<T: AsRef<Path>>(path: T) -> Result<String> {
    if path.as_ref().is_file() {
        Ok(path
            .as_ref()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned())
    } else {
        Err(Error::from("invalid file path"))
    }
}
