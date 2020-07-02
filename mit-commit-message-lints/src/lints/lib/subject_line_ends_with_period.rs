use indoc::indoc;
use mit_commit::CommitMessage;

use crate::lints::lib::problem::Code;
use crate::lints::lib::Problem;

pub(crate) const CONFIG: &str = "subject-line-ends-with-period";

const HELP_MESSAGE: &str = indoc!(
    "
    Your commit message ends with a period

    You can fix this by removing the period
    "
);

fn has_problem(commit_message: &CommitMessage) -> bool {
    if let Some('.') = commit_message.get_subject().chars().rev().next() {
        true
    } else {
        false
    }
}

pub(crate) fn lint(commit_message: &CommitMessage) -> Option<Problem> {
    if has_problem(&commit_message) {
        Some(Problem::new(
            HELP_MESSAGE.into(),
            Code::SubjectEndsWithPeriod,
        ))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::wildcard_imports)]

    use indoc::indoc;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn subject_does_not_end_with_period() {
        run_test(
            indoc!(
                "
                Subject Line
                "
            ),
            &None,
        );
    }

    #[test]
    fn subject_ends_with_period() {
        run_test(
            indoc!(
                "
                Subject Line.
                "
            ),
            &Some(Problem::new(
                HELP_MESSAGE.into(),
                Code::SubjectEndsWithPeriod,
            )),
        );
    }

    #[test]
    fn subject_has_period_then_whitespace() {
        run_test(
            indoc!(
                "
                Subject Line.
                "
            ),
            &Some(Problem::new(
                HELP_MESSAGE.into(),
                Code::SubjectEndsWithPeriod,
            )),
        );
    }

    fn run_test(message: &str, expected: &Option<Problem>) {
        let actual = &lint(&CommitMessage::from(message));
        assert_eq!(
            actual, expected,
            "Message {:?} should have returned {:?}, found {:?}",
            message, expected, actual
        );
    }
}