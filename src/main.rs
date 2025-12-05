use clap::{Arg, Command};

mod paths;
mod models;
mod util {
  pub mod api;
  pub mod download;
  pub mod extract;
}

mod commands {
  pub mod list;
  pub mod list_remote;
  pub mod install;
  pub mod use_version;
  pub mod remove;
  pub mod env;
}

fn main() {
  let matches = Command::new("jman")
    .version("0.1.0")
    .about("OpenJDK version manager")
    .subcommand_required(true)
    .arg_required_else_help(true)
    .subcommand(
      Command::new("list")
        .about("List installed versions")
    )
    .subcommand(
      Command::new("list-remote")
        .about("List remote versions")
    )
    .subcommand(
      Command::new("use")
        .about("Select a version to use")
        .arg(
          Arg::new("version")
            .help("Version name")
            .required(true)
            .index(1),
        ),
    )
    .subcommand(
      Command::new("install")
        .about("Select a version to install")
        .arg(
          Arg::new("version")
            .help("Version name")
            .required(true)
            .index(1),
        ),
    )
    .subcommand(
      Command::new("remove")
        .about("Select a version to remove")
        .arg(
          Arg::new("version")
            .help("Version name")
            .required(true)
            .index(1),
        ),
    )
    .get_matches();

  match matches.subcommand() {
    Some(("list", _)) => commands::list::list_versions(),
    Some(("list-remote", _)) => commands::list_remote::list_versions_remote(),
    Some(("use", arg)) => {
      let version = arg.get_one::<String>("version").unwrap();
      commands::use_version::use_version(version);
      commands::env::set_env();
    }
    Some(("install", arg)) => {
      let version = arg.get_one::<String>("version").unwrap();
      commands::install::install_version(version);
    }
    Some(("remove", arg)) => {
      let version = arg.get_one::<String>("version").unwrap();
      commands::remove::remove_version(version);
    }
    _ => {},
  }
}
