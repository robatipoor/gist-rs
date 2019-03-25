use clap::{crate_authors, crate_description, crate_name, crate_version};
use clap::{App, Arg, SubCommand};

#[derive(Debug)]
pub enum Mod {
    Get,
    Create,
    Delete,
    Update,
}
#[derive(Default, Debug)]
pub struct AppArgs {
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
    pub usage: Option<String>,
}

pub fn get_args() -> Option<AppArgs> {
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
            ]),
        )
        .subcommand(
            SubCommand::with_name("create").about("create gist").args(&[
                Arg::with_name("path")
                    .short("p")
                    .long("path")
                    .value_name("PATH")
                    .help("create gist")
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
                Arg::with_name("id")
                    .short("i")
                    .long("id")
                    .value_name("ID")
                    .help("Sets a ID gist ")
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

    let mut app_args: AppArgs = AppArgs::default();
    if let Some(login) = matches.value_of("login") {
        app_args.login = Some(login.to_owned());
        return Some(app_args);
    }

    if let Some(m) = matches.subcommand_matches("get") {
        app_args.rmod = Some(Mod::Get);
        if let Some(i) = m.value_of("id") {
            app_args.id = Some(i.to_owned());
        } else if let Some(u) = m.value_of("url") {
            app_args.url = Some(u.to_owned());
        }
    } else if let Some(m) = matches.subcommand_matches("create") {
        app_args.rmod = Some(Mod::Create);
        app_args.public = m.is_present("public");
        if let Some(p) = m.value_of("path") {
            app_args.path = Some(p.to_owned());
        }
        if let Some(d) = m.value_of("desc") {
            app_args.desc = Some(d.to_owned());
        }
        if let Some(n) = m.value_of("name") {
            app_args.name = Some(n.to_owned());
        }
    } else if let Some(m) = matches.subcommand_matches("update") {
        app_args.rmod = Some(Mod::Update);
        app_args.public = m.is_present("public");
        if let Some(i) = m.value_of("id") {
            app_args.id = Some(i.to_owned());
        }
        if let Some(d) = m.value_of("desc") {
            app_args.desc = Some(d.to_owned());
        }
        if let Some(u) = m.value_of("url") {
            app_args.url = Some(u.to_owned());
        }
        if let Some(p) = m.value_of("path") {
            app_args.path = Some(p.to_owned());
        }
        if let Some(n) = m.value_of("name") {
            app_args.name = Some(n.to_owned());
        }
    } else if let Some(m) = matches.subcommand_matches("delete") {
        app_args.rmod = Some(Mod::Delete);
        if let Some(i) = m.value_of("id") {
            app_args.id = Some(i.to_owned());
        } else if let Some(u) = m.value_of("url") {
            app_args.url = Some(u.to_owned());
        }
    } else {
        if matches.is_present("list") {
            app_args.list = true;
            app_args.verbose = matches.is_present("verbose");
        }
        app_args.sync = matches.is_present("sync");
        if !(app_args.list || app_args.sync) {
            return None;
        }
    }
    app_args.usage = matches.usage;
    return Some(app_args);
}
