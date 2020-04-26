use git2::Repository;
use pretty_assertions::assert_eq;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Output};
use std::{env, str};
use tempfile::{NamedTempFile, TempDir};

fn calculate_cargo_toml_path() -> String {
    env::current_exe()
        .unwrap()
        .parent()
        .and_then(std::path::Path::parent)
        .and_then(std::path::Path::parent)
        .and_then(std::path::Path::parent)
        .map(|x| x.join("pb-commit-msg").join("Cargo.toml"))
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()
}

#[test]
fn duplicated_trailers_cause_errors() {
    let input = r#"An example commit

This is an example commit with duplicate trailers

Signed-off-by: Billie Thompson <email@example.com>
Signed-off-by: Billie Thompson <email@example.com>
"#;

    let working_dir = setup_working_dir();
    let output = run_hook(input, &working_dir);
    let stdout = str::from_utf8(&output.stdout).unwrap();
    assert!(
        stdout.is_empty(),
        "Expected stdout to be empty, instead it contained \"{}\"",
        stdout
    );

    let stderr = str::from_utf8(&output.stderr).unwrap();
    let expected_stderr = r#"
An example commit

This is an example commit with duplicate trailers

Signed-off-by: Billie Thompson <email@example.com>
Signed-off-by: Billie Thompson <email@example.com>


Your commit cannot have the same name duplicated in the "Signed-off-by" field

"#;
    assert_eq!(
        stderr.to_string(),
        expected_stderr,
        "Expected stderr to contain {:?}, instead it contained {:?}",
        expected_stderr,
        stderr
    );

    assert!(
        !&output.status.success(),
        "Expected status to be a failure, instead it was {}",
        &output.status.code().unwrap()
    );
}

fn run_hook(fake_commit_message: &str, working_dir: &PathBuf) -> Output {
    let mut commit_path = NamedTempFile::new().unwrap();
    write!(commit_path, "{}", fake_commit_message).unwrap();

    Command::new("cargo")
        .current_dir(&working_dir)
        .arg("run")
        .arg("--quiet")
        .arg("--manifest-path")
        .arg(calculate_cargo_toml_path())
        .arg("--")
        .arg(commit_path.path().to_str().unwrap())
        .output()
        .expect("failed to execute process")
}

fn setup_working_dir() -> PathBuf {
    let temp = TempDir::new()
        .map(|x| x.into_path().join("repository"))
        .expect("Unable to make path");
    Repository::init(&temp).expect("Couldn't create repo");

    temp
}

#[test]
fn a_valid_commit_is_fine() {
    let input = r#"An example commit

This is an example commit with duplicate trailers

Signed-off-by: Billie Thompson <email@example.com>
"#;

    let working_dir = setup_working_dir();
    let output = run_hook(input, &working_dir);
    let stdout = str::from_utf8(&output.stdout).unwrap();
    assert!(
        stdout.is_empty(),
        "Expected stdout to be empty, instead it contained \"{}\"",
        stdout
    );

    let stderr = str::from_utf8(&output.stderr).unwrap();
    assert!(
        stderr.is_empty(),
        "Expected stderr to be empty, instead it contained \"{}\"",
        stderr
    );

    assert!(
        &output.status.success(),
        "Expected status to be successful, instead it was {}",
        &output.status.code().unwrap()
    );
}

#[test]
fn i_can_disable_the_check() {
    let input = r#"An example commit

This is an example commit with duplicate trailers

Signed-off-by: Billie Thompson <email@example.com>
Signed-off-by: Billie Thompson <email@example.com>
"#;
    let working_dir = setup_working_dir();
    Command::new("git")
        .current_dir(&working_dir)
        .arg("config")
        .arg("--local")
        .arg("pb.message.duplicated-trailers")
        .arg("false")
        .output()
        .expect("failed to execute process");
    let output = run_hook(input, &working_dir);

    let stdout = str::from_utf8(&output.stdout).expect("stdout couldn't be parsed");
    let stderr = str::from_utf8(&output.stderr).expect("stderr couldn't be parsed");
    assert!(
        stdout.is_empty(),
        "Expected stderr to be empty, instead it contained {:?} stderr {:?} status {:?}",
        stdout,
        stderr,
        output.status.code()
    );

    assert!(
        stderr.is_empty(),
        "Expected stderr to be empty, instead it contained {:?} stdout {:?} status {:?}",
        stderr,
        stdout,
        output.status.code()
    );

    assert!(
        &output.status.success(),
        "Expected status to be successful, instead it was {:?}  stdout {:?} stderr {:?}",
        &output.status.code(),
        stdout,
        stderr
    );
}