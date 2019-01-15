use clap::{crate_authors, crate_description, crate_name, crate_version};
use clap::{App, Arg, SubCommand};

#[derive(Debug)]
pub enum Mod {
    Get,
    Post,
    Del,
    Update,
}
#[derive(Default, Debug)]
pub struct Config {
    pub list: bool,
    pub verbose: bool,
    pub sync: bool,
    pub public: bool,
    pub rmod: Option<Mod>,
    pub login: Option<String>,
    pub id: Option<String>,
    pub url: Option<String>,
    pub desc: Option<String>,
    pub path: Option<String>,
    pub name: Option<String>,
}

pub fn get_config() -> Config {
    let matches = App::new(crate_name!())
        .about(crate_description!())
        .version(crate_version!())
        .author(crate_authors!())
        .args(&[
            Arg::with_name("login")
                .short("L")
                .long("login")
                .value_name("TOKEN")
                .help("login github token")
                .takes_value(true),
            Arg::with_name("list")
                .short("l")
                .long("list")
                .help("list of gists"),
            Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .help("show verbose list of gists"),
            Arg::with_name("sync")
                .short("s")
                .long("sync")
                .help("sync list of gists"),
        ])
        .subcommand(
            SubCommand::with_name("get").about("get gist").args(&[
                Arg::with_name("url")
                    .short("u")
                    .long("url")
                    .value_name("RAW URL")
                    .help("Sets a url gist ")
                    .takes_value(true),
                Arg::with_name("id")
                    .short("i")
                    .long("id")
                    .value_name("ID")
                    .help("Sets a ID gist ")
                    .takes_value(true),
                Arg::with_name("desc")
                    .short("d")
                    .long("desc")
                    .value_name("description gist")
                    .help("Sets a ID gist ")
                    .takes_value(true),
            ]),
        )
        .subcommand(
            SubCommand::with_name("post").about("post gist").args(&[
                Arg::with_name("path")
                    .short("p")
                    .long("path")
                    .value_name("PATH")
                    .help("upload gist")
                    .takes_value(true),
                Arg::with_name("desc")
                    .short("d")
                    .long("desc")
                    .value_name("DESCRIPTION")
                    .help("description gist")
                    .takes_value(true),
                Arg::with_name("public")
                    .short("P")
                    .long("public")
                    .help("Sets a gist url")
                    .takes_value(false),
                Arg::with_name("name")
                    .short("n")
                    .long("name")
                    .value_name("NAME FILE")
                    .help("name file gist")
                    .takes_value(true),
            ]),
        )
        .subcommand(
            SubCommand::with_name("update").about("update gist").args(&[
                Arg::with_name("url")
                    .short("u")
                    .long("url")
                    .value_name("URL")
                    .help("Sets a gist url")
                    .takes_value(true),
                Arg::with_name("path")
                    .short("p")
                    .long("path")
                    .value_name("PATH")
                    .help("upload gist")
                    .takes_value(true),
                Arg::with_name("desc")
                    .short("d")
                    .long("desc")
                    .value_name("DESCRIPTION")
                    .help("description gist")
                    .takes_value(true),
                Arg::with_name("name")
                    .short("n")
                    .long("name")
                    .value_name("NAME FILE")
                    .help("name file gist")
                    .takes_value(true),
            ]),
        )
        .subcommand(
            SubCommand::with_name("delete").about("delete gist").args(&[
                Arg::with_name("url")
                    .short("u")
                    .long("url")
                    .value_name("URL")
                    .help("Sets a url gist ")
                    .takes_value(true),
                Arg::with_name("id")
                    .short("i")
                    .long("id")
                    .value_name("ID")
                    .help("Sets a ID gist ")
                    .takes_value(true),
            ]),
        )
        .get_matches();

    let mut config: Config = Config::default();
    if let Some(login) = matches.value_of("login") {
        config.login = Some(login.to_owned());
        return config;
    }
    config.list = matches.is_present("list");
    config.verbose = matches.is_present("verbose");
    config.sync = matches.is_present("sync");

    if let Some(m) = matches.subcommand_matches("get") {
        config.rmod = Some(Mod::Get);
        if let Some(i) = m.value_of("id") {
            config.id = Some(i.to_owned());
        } else if let Some(d) = m.value_of("desc") {
            config.desc = Some(d.to_owned());
        } else if let Some(u) = m.value_of("url") {
            config.url = Some(u.to_owned());
        }
    } else if let Some(m) = matches.subcommand_matches("post") {
        config.rmod = Some(Mod::Post);
        config.public = m.is_present("public");
        if let Some(p) = m.value_of("path") {
            config.path = Some(p.to_owned());
        }
        if let Some(d) = m.value_of("desc") {
            config.desc = Some(d.to_owned());
        }
        if let Some(n) = m.value_of("name") {
            config.name = Some(n.to_owned());
        }
    } else if let Some(m) = matches.subcommand_matches("update") {
        config.rmod = Some(Mod::Update);
        config.public = m.is_present("public");
        if let Some(i) = m.value_of("id") {
            config.id = Some(i.to_owned());
        }
        if let Some(d) = m.value_of("desc") {
            config.desc = Some(d.to_owned());
        }
        if let Some(u) = m.value_of("url") {
            config.url = Some(u.to_owned());
        }
        if let Some(p) = m.value_of("path") {
            config.path = Some(p.to_owned());
        }
        if let Some(n) = m.value_of("name") {
            config.name = Some(n.to_owned());
        }
    } else if let Some(m) = matches.subcommand_matches("delete") {
        config.rmod = Some(Mod::Del);
        if let Some(i) = m.value_of("id") {
            config.id = Some(i.to_owned());
        } else if let Some(u) = m.value_of("url") {
            config.url = Some(u.to_owned());
        }
    }

    return config;
}
