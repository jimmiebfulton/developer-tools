use clap::{command, Arg, Command, ArgAction, crate_name};
use clap::builder::{EnumValueParser, PathBufValueParser};
use crate::installers::InstallerKey;

pub fn command() -> Command {
    command!()
        .name(crate_name!())
        .subcommand_required(true)
        .arg(
            Arg::new("verbosity")
                .short('v')
                .action(ArgAction::Count)
                .global(true)
        )
        .subcommand(
            Command::new("git")
                .subcommand_required(true)
                .about("git operations")
                .subcommand(
                    Command::new("init")
                        .about("Initialize a Git Repo")
                        .arg(
                            Arg::new("personal")
                                .help("Configure Personal Identity")
                                .short('p')
                                .action(ArgAction::SetTrue)
                        )
                )
                .subcommand(
                    Command::new("add-remote")
                        .about("Initialize a Git Repo")
                        .arg(
                            Arg::new("url")
                                .help("Git URL")
                                .required(true)
                                .action(ArgAction::Set)
                        )
                        .arg(
                            Arg::new("personal")
                                .help("Configure Personal URL")
                                .short('p')
                                .action(ArgAction::SetTrue)
                        )
                )
                .subcommand(
                    Command::new("clone")
                        .about("Initialize a Git Repo")
                        .arg(
                            Arg::new("url")
                                .help("Git URL")
                                .required(true)
                                .action(ArgAction::Set)
                        )
                        .arg(
                            Arg::new("destination")
                                .help("Destination")
                                .action(ArgAction::Set)
                        )
                        .arg(
                            Arg::new("personal")
                                .help("Configure Personal URL")
                                .short('p')
                                .action(ArgAction::SetTrue)
                        )
                )
        )

        .subcommand(
        Command::new("rd")
            .about("Remove Directory")
            .arg_required_else_help(true)
            .arg(
                Arg::new("paths")
                    .help("Path to delete")
                    .required(true)
                    .action(ArgAction::Append)
                    .value_parser(PathBufValueParser::new()),
            )
        )

        .subcommand(
            Command::new("init")
                .about("Generate fish abbreviations")
                .subcommand_required(true)
                .subcommand(
                    Command::new("fish")
                        .about("Initialize Fish Shell")

                )
        )

        .subcommand(
            Command::new("install")
                .about("Install")
                .arg(
                    Arg::new("installer")
                        .value_parser(EnumValueParser::<InstallerKey>::new())
                        .required(true)
                )
        )

        .subcommand(
            Command::new("update")
                .about("Update")
                .arg(
                    Arg::new("installer")
                        .value_parser(EnumValueParser::<InstallerKey>::new())
                        .required(true)
                )
        )
        .subcommand(
            Command::new("bootstrap")
                .about("Bootstrap System")
        )
}