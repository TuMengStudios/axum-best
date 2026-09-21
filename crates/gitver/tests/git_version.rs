#[test]
fn matches_latest_git_log() {
    const COMMIT: &str = gitver::git_version!();
    const COMMIT_WITH_FALLBACK: &str = gitver::git_version!(fallback = "unknown");

    let output = std::process::Command::new("git")
        .args(["log", "-1", "--format=%h"])
        .output()
        .expect("failed to run git");
    let expected = String::from_utf8(output.stdout)
        .expect("invalid utf-8")
        .trim()
        .to_owned();

    assert_eq!(COMMIT, expected);
    assert_eq!(COMMIT_WITH_FALLBACK, expected);
}
