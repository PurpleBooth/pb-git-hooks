use std::convert::TryInto;

use clap::ArgMatches;
use comfy_table::Table;

use mit_commit_message_lints::external;
use mit_commit_message_lints::lints::Lints;

use crate::errors::GitMitConfigError;
use crate::errors::GitMitConfigError::LintNameNotGiven;
use crate::{current_dir, get_vcs};

pub(crate) fn run_on_match(matches: &ArgMatches) -> Option<Result<(), GitMitConfigError>> {
    matches
        .subcommand_matches("lint")
        .filter(|subcommand| subcommand.subcommand_matches("status").is_some())
        .map(|_| run(matches))
}

fn run(matches: &ArgMatches) -> Result<(), GitMitConfigError> {
    let subcommand_args = matches
        .subcommand_matches("lint")
        .and_then(|x| x.subcommand_matches("status"))
        .unwrap();
    let is_local = Some("local") == matches.value_of("scope");
    let current_dir = current_dir()?;
    let mut vcs = get_vcs(is_local, &current_dir)?;
    let toml = external::read_toml(current_dir)?;

    let lints = get_selected_lints(&subcommand_args)?;
    let config = Lints::get_from_toml_or_else_vcs(&toml, &mut vcs)?;
    let status = get_config_status(lints.clone(), config);
    let names = lints.names();

    let mut table = Table::new();
    table.set_header(vec!["Lint", "Status"]);

    table = names
        .into_iter()
        .zip(status)
        .fold(table, |mut table, (name, status_name)| {
            table.add_row(vec![name, status_name]);
            table
        });

    println!("{}", table);

    Ok(())
}

fn get_config_status<'a>(lints: Lints, config: Lints) -> Vec<&'a str> {
    let enabled_lints: Vec<_> = config.into_iter().collect();

    lints
        .into_iter()
        .map(|lint| {
            if enabled_lints.contains(&lint) {
                "enabled"
            } else {
                "disabled"
            }
        })
        .collect::<Vec<_>>()
}

fn get_selected_lints(args: &ArgMatches) -> Result<Lints, GitMitConfigError> {
    Ok(args
        .values_of("lint")
        .ok_or_else(|| LintNameNotGiven)?
        .collect::<Vec<_>>()
        .try_into()?)
}
