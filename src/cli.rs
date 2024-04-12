use clap::{command, Arg, Command, value_parser, ArgAction};
use clap_complete::Shell;

pub fn command() -> Command {
    command!()
        .name("dt")
        .subcommand_required(true)
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
        )
        .subcommand(
            Command::new("completions")
                .about("Generate shell completions")
                .arg_required_else_help(true)
                .arg(
                    Arg::new("generator")
                        .value_parser(value_parser!(Shell)),
                )
        )
        .subcommand(
            Command::new("aliases")
                .about("Generate fish abbreviations")
        )
}