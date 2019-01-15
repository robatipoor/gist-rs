mod cli;
mod delete;
mod errors;
mod get;
mod list_gist;
mod post;
mod token;
mod update;
mod url;
use crate::cli::*;
use crate::list_gist::*;
use crate::token::*;
use crate::url::*;
use std::io::prelude::*;
use std::path::Path;

#[cfg(test)]
mod tests;

fn main() {
    let config: Config = get_config();
    let path_gist = path_gist_file().unwrap();
    if let Some(l) = config.login {
        write_token(l).unwrap();
        println!("login successfully");
        return;
    }
    if config.list && config.sync {
        write_file(&path_gist_file().unwrap(), sync_list().unwrap()).unwrap();
        print_list(read_list_gists(&path_gist).unwrap(), config.verbose).unwrap();
        return;
    }
    if config.list {
        print_list(read_list_gists(&path_gist).unwrap(), config.verbose).unwrap();
        return;
    }
    if config.sync {
        write_file(&path_gist_file().unwrap(), sync_list().unwrap()).unwrap();
        return;
    }
    if let Some(m) = config.rmod {
        match m {
            Mod::Get => {
                let out;
                if let Some(u) = config.url {
                    out = get::get_gist(&u).unwrap();
                } else if let Some(i) = config.id {
                    out = get::get_gist(&search_raw_url(&path_gist, &i).unwrap()).unwrap();
                } else {
                    eprintln!("invalid input !");
                    return;
                }
                println!("{}", out);
            }
            Mod::Del => {
                if let Some(u) = config.url {
                    delete::delete_gist(&u).unwrap();
                } else if let Some(i) = config.id {
                    delete::delete_gist(&search_url(&path_gist, &i).unwrap()).unwrap();
                } else {
                    eprintln!("invalid input !");
                }
                println!("delete gist successfully !");
            }
            Mod::Post => {
                if let Some(p) = config.path {
                    let p = Path::new(&p);
                    let res = post::GistPost::new(
                        read_file(&p).unwrap(),
                        config.public,
                        config.desc.unwrap_or("".to_owned()),
                        config.name.unwrap_or(get_name_file(&p).unwrap()),
                    )
                    .post()
                    .unwrap();
                    println!("{:?}", res)
                } else {
                    let mut i = std::io::stdin();
                    let mut buf = String::new();
                    i.read_to_string(&mut buf).expect("failed read stdin");
                    let res = post::GistPost::new(
                        buf,
                        config.public,
                        config.desc.unwrap_or("".to_owned()),
                        config.name.unwrap_or("".to_owned()),
                    )
                    .post()
                    .unwrap();
                    println!("{:?}", res)
                }
            }
            Mod::Update => {
                if let Some(p) = config.path {
                    if let Some(u) = config.url {
                        let p = Path::new(&p);
                        let name = config.name.unwrap_or(get_name_file(&p).unwrap());
                        let res = update::GistUpdate::new(
                            read_file(p).unwrap(),
                            config.desc.unwrap_or("".to_owned()),
                            name.clone(), // TODO FIX old name file
                            Some(name),
                        )
                        .update(&u)
                        .unwrap();
                        println!("{:?}", res)
                    }
                }
            }
        }
    }
}
