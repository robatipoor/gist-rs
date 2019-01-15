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
        println!("sync gist list successfully !");
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
                    delete::delete_gist(&convert_url(&path_gist, &u).unwrap()).unwrap();
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
                    let _res = post::GistPost::new(
                        read_file(&p).unwrap(),
                        config.public,
                        config.desc.unwrap_or("".to_owned()),
                        config.name.unwrap_or(get_name_file(&p).unwrap()),
                    )
                    .post()
                    .unwrap();
                    println!("post gist successfully !");
                } else {
                    let mut i = std::io::stdin();
                    let mut buf = String::new();
                    i.read_to_string(&mut buf).expect("failed read stdin");
                    let _res = post::GistPost::new(
                        buf,
                        config.public,
                        config.desc.unwrap_or("".to_owned()),
                        config.name.unwrap_or("".to_owned()),
                    )
                    .post()
                    .unwrap();
                    println!("post gist successfully!");
                }
            }
            Mod::Update => {
                if let Some(p) = config.path {
                    if let Some(i) = config.id {
                        let p = Path::new(&p);
                        let name = config.name.unwrap_or(get_name_file(&p).unwrap());
                        let _res = update::GistUpdate::new(
                            read_file(p).unwrap(),
                            config.desc.unwrap_or("".to_owned()),
                            get_name_file_from_id(&path_gist, &i).unwrap(), // TODO FIX old name file
                            Some(name),
                        )
                        .update(&search_url(&path_gist, &i).unwrap())
                        .unwrap();
                        println!("update gist successfully !");
                    } else if let Some(u) = config.url {
                        let p = Path::new(&p);
                        let _res = update::GistUpdate::new(
                            read_file(p).unwrap(),
                            config.desc.unwrap_or("".to_owned()),
                            get_name_file_from_url(&path_gist, &u).unwrap(), // TODO FIX old name file
                            Some(config.name.unwrap_or(get_name_file(&p).unwrap())),
                        )
                        .update(&convert_url(&path_gist, &u).unwrap())
                        .unwrap();
                        println!("update gist successfully !");
                    }
                }
            }
        }
    } else {
        println!("{}", config.usage.unwrap())
    }
}
