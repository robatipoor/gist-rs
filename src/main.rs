mod cli;
mod constants;
mod delete;
mod errors;
mod get_gist;
mod list_gist;
mod post;
mod token;
mod update;
mod utils;

use cli::*;
use list_gist::*;
use std::path::Path;
use token::*;

#[cfg(test)]
mod tests;

fn main() {
    let app_args = get_args().unwrap_or_else(|| {
        let _ = utils::read_stdin();
        std::process::exit(0);
    });
    if let Some(l) = app_args.login {
        Token::write(l).unwrap();
        println!("login successfully");
        return;
    }
    if app_args.list && app_args.sync {
        ListGist::sync().unwrap().print(app_args.verbose).unwrap();
        return;
    }
    if app_args.list {
        ListGist::read().unwrap().print(app_args.verbose).unwrap();
        return;
    }
    if app_args.sync {
        ListGist::sync().unwrap();
        println!("sync gist list successfully !");
        return;
    }
    if let Some(m) = app_args.rmod {
        let list_gist = ListGist::read().unwrap();
        match m {
            Mod::Get => {
                let out;
                if let Some(i) = app_args.id {
                    let url = list_gist.search_raw_url_gist(i).unwrap();
                    out = get_gist::get_gist_file(&*url).unwrap();
                } else {
                    eprintln!("invalid input !");
                    return;
                }
                println!("{}", out);
            }
            Mod::Delete => {
                if let Some(i) = app_args.id {
                    delete::delete_gist(&list_gist.search_raw_url_gist(&i).unwrap()).unwrap();
                } else {
                    eprintln!("invalid input !");
                }
                println!("delete gist successfully !");
            }
            Mod::Create => {
                if let Some(p) = app_args.path {
                    let p = Path::new(&p);
                    let _res = post::GistPost::new(
                        utils::read_file(&p).unwrap(),
                        app_args.public,
                        app_args.desc.unwrap_or("".to_owned()),
                        app_args.name.unwrap_or(utils::get_name_file(&p).unwrap()),
                    )
                    .post()
                    .unwrap();
                    println!("post gist successfully !");
                }
            }
            Mod::Update => {
                if let Some(p) = app_args.path {
                    if let Some(i) = app_args.id {
                        let p = Path::new(&p);
                        let name = app_args.name.unwrap_or(utils::get_name_file(&p).unwrap());
                        let _res = update::GistUpdate::new(
                            utils::read_file(p).unwrap(),
                            app_args.desc.unwrap_or("".to_owned()),
                            list_gist.get_name_gist_file(&i).unwrap(), // TODO FIX old name file
                            Some(name),
                        )
                        .update(list_gist.search_raw_url_gist(i).unwrap().as_str())
                        .unwrap();
                        println!("update gist successfully !");
                    } else {
                        eprintln!("invalid input !");
                        return;
                    }
                }
            }
        }
    } else {
        if let Some(s) = utils::read_stdin() {
            let _ = post::GistPost::new(
                s,
                app_args.public,
                app_args.desc.unwrap_or("".to_owned()),
                app_args.name.unwrap_or("".to_owned()),
            )
            .post()
            .unwrap();
            println!("post gist successfully!");
        } else {
            println!("{}", app_args.usage.unwrap())
        }
    }
}
