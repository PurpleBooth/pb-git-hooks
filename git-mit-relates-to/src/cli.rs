use clap::{crate_authors, crate_version, App, Arg};

pub fn app() -> App<'static> {
    App::new(String::from(env!("CARGO_PKG_NAME")))
        .bin_name(String::from(env!("CARGO_PKG_NAME")))
        .version(crate_version!())
        .author(crate_authors!())
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::new("issue-number")
                .about("The issue number or other string to place into the Relates-to trailer")
                .required(true),
        )
        .arg(
            Arg::new("timeout")
                .short('t')
                .long("timeout")
                .about("Number of minutes to expire the configuration in")
                .env("GIT_MIT_RELATES_TO_TIMEOUT")
                .default_value("60"),
        )
}
