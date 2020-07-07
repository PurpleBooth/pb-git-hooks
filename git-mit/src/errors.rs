use std::string;

use thiserror::Error;

use mit_commit_message_lints::{external, mit};

#[derive(Error, Debug)]
pub enum GitMitError {
    #[error("failed convert to unicode: {0}")]
    Utf8(#[from] string::FromUtf8Error),
    #[error("failed to interact with git repository: {0}")]
    Git2Io(#[from] external::Error),
    #[error("no mit initials provided")]
    NoAuthorInitialsProvided,
    #[error("no timeout set")]
    NoTimeoutSet,
    #[error("timeout needs to be the number of minutes: {0}")]
    TimeoutNotNumber(#[from] std::num::ParseIntError),
    #[error("expected a mit file path, didn't find one")]
    AuthorFileNotSet,
    #[error("failed to read config from `{0}`: {1}")]
    Io(String, String),
    #[error("failed to generate config with `{0}`: {1}")]
    Exec(String, String),
    #[error("failed to calculate config directory {0}")]
    Xdg(#[from] xdg::BaseDirectoriesError),
    #[error("failed to parse mit yaml {0}")]
    AuthorYaml(#[from] mit::YamlError),
    #[error("failed to set mit in vcs {0}")]
    AuthorVcs(#[from] mit::VcsError),
}

impl GitMitError {
    pub(crate) fn new_io(source: String, error: &std::io::Error) -> GitMitError {
        GitMitError::Io(source, format!("{}", error))
    }

    pub(crate) fn new_exec(source: String, error: &std::io::Error) -> GitMitError {
        GitMitError::Exec(source, format!("{}", error))
    }
}
