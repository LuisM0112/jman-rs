use clap::{Arg, Command};
use std::fs;

mod paths;
mod models;
mod util {
  pub mod download;
  pub mod extract;
}

mod commands {
  pub mod list;
  pub mod list_remote;
  pub mod install;
  pub mod use_version;
  pub mod remove;
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
      set_env();
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

fn set_env() {
  // #[cfg(unix)]
  {
    let env_file = paths::base_dir().join("env.sh");

    let content = format!(
      "export JAVA_HOME=\"{}/current\"\nexport PATH=\"$JAVA_HOME/bin:$PATH\"\n",
      paths::base_dir().display()
    );
  
    if let Err(e) = fs::write(&env_file, content) {
      eprintln!("Failed to write env file: {}", e);
      return;
    }

    let bashrc = paths::home_dir().join(".bashrc");
    let line = "source \"$HOME/.jman/env.sh\"";
    let bashrc_content = fs::read_to_string(&bashrc).unwrap_or_default();

    if bashrc_content.contains(line) {
      return;
    }

    let updated = format!("{}\n{}", bashrc_content, line);

    if let Err(e) = fs::write(&bashrc, updated) {
      eprintln!("Failed to write in bashrc: {}", e);
      return;
    }

    println!("Environment variables loaded into ~/.bashrc");
    println!("To use this java version on this session run: . ~/.jman/env.sh")
  }
}
