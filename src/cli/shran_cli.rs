use super::commands::{SubCommandName, ArgName, ActiveCommand};
use crate::error::{ShranError, ShranErrorType};
use clap::{
    crate_authors, crate_description, crate_name, crate_version, App, AppSettings, Arg, ArgMatches,
    SubCommand,
};
use std::path::Path;


/// Wrapper around the clap command line interface library.
///
/// # Example
///
/// ```no_run
/// let cli = Cli::new().unwrap_or_else(|error: ShranError| {
///     eprintln!("{}", error);
///     std::process::exit(1);
/// });
/// ```
#[derive(Debug)]
pub struct Cli {
    active_command: ActiveCommand,
}

impl<'ebuf> Cli {
    pub fn new() -> ShranErrorType<'ebuf, Self> {
        let m: ArgMatches = App::new(crate_name!())
            .author(crate_authors!())
            .version(crate_version!())
            .about(crate_description!())
            .setting(AppSettings::SubcommandRequiredElseHelp)
            .subcommand(
                SubCommand::with_name(SubCommandName::GENERATE)
                    .about("Generate a build configuration for a specified blockchain")
                    .setting(AppSettings::ArgRequiredElseHelp)
                    .arg(
                        Arg::with_name(ArgName::BITCOIN)
                            .long("btc")
                            .help("Generates a build.yaml configuration file to build the bitcoin source code"),
                    )
                    .arg(
                        Arg::with_name(ArgName::LITECOIN)
                            .long("ltc")
                            .help("Generates a build.yaml configuration file to build the litecoin source code"),
                    ),
            )
            .subcommand(
                SubCommand::with_name(SubCommandName::BUILD)
                    .about("Build ")
                    .setting(AppSettings::ArgRequiredElseHelp)
                    .arg(
                        Arg::with_name(ArgName::CONFIG)
                            .short("c")
                            .long("config")
                            .takes_value(true)
                            .help("Path to an existing build configuration"),
                    ),
            )
            .subcommand(
                SubCommand::with_name(SubCommandName::AUTH)
                    .about("Authorize api access to your github account")
                    .setting(AppSettings::ArgRequiredElseHelp)
                    .arg(
                        Arg::with_name(ArgName::TOKEN)
                            .long("with-token")
                            .takes_value(true)
                            .help("Your github token"),
                    ),
            )
            .get_matches();

        let active_command = Self::build_active_command(&m)?;

        Ok(Self { active_command })
    }

    pub fn active_command(&self) -> &ActiveCommand {
        &self.active_command
    }

    fn build_active_command(matches: &ArgMatches) -> ShranErrorType<'ebuf, ActiveCommand> {
        match matches.subcommand() {
            (SubCommandName::GENERATE, Some(generate_matches)) => {
                let arg = generate_matches.value_of(ArgName::BITCOIN).unwrap();
                Ok(ActiveCommand::new(SubCommandName::GENERATE, arg))
            }
            (SubCommandName::BUILD, Some(build_matches)) => {
                let arg = build_matches.value_of(ArgName::CONFIG).unwrap();
                if !Path::new(&arg).exists() {
                    return Err(ShranError::BuildFileError {
                        msg: arg.to_string(),
                        file: file!(),
                        line: line!(),
                    });
                }
                Ok(ActiveCommand::new(SubCommandName::BUILD, arg))
            }
            (SubCommandName::AUTH, Some(auth_matches)) => {
                let arg = auth_matches.value_of("token").unwrap();
                Ok(ActiveCommand::new(SubCommandName::AUTH, arg))
            }
            _ => unreachable!(),
        }
    }
}
