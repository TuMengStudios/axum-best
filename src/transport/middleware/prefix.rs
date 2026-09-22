//! Path exclusion matching shared by the middlewares' per-path exclusions.
//!
//! Plain entries are segment-aware prefixes: `/stream` excludes `/stream` and
//! `/stream/1` but not `/streaming`, and a bare `/` excludes everything.
//! Entries starting with `regex:` are instead treated as regular expressions
//! matched against the full request path, e.g. `regex:^/files/.+\.zip$`.
//! Invalid regexes panic at startup instead of silently never matching.

use regex::Regex;

/// One configured exclusion: a segment-aware path prefix or a precompiled
/// regular expression (entries starting with `regex:`)
pub(super) enum Exclusion {
    Prefix(String),
    Pattern(Regex),
}

impl Exclusion {
    /// Parses one configured entry; invalid regexes fail fast at startup
    pub(super) fn new(entry: &str) -> Exclusion {
        match entry.strip_prefix("regex:") {
            Some(pattern) => Exclusion::Pattern(
                Regex::new(pattern)
                    .unwrap_or_else(|err| panic!("invalid exclusion regex {entry:?}: {err}")),
            ),
            None => Exclusion::Prefix(entry.trim_end_matches('/').to_owned()),
        }
    }

    /// Returns whether `path` matches this exclusion
    pub(super) fn matches(&self, path: &str) -> bool {
        match self {
            Exclusion::Prefix(prefix) => path == prefix || path.starts_with(&format!("{prefix}/")),
            Exclusion::Pattern(pattern) => pattern.is_match(path),
        }
    }
}

/// Returns whether `path` matches any of the configured exclusions
pub(super) fn is_excluded(path: &str, exclusions: &[Exclusion]) -> bool {
    exclusions.iter().any(|exclusion| exclusion.matches(path))
}

#[cfg(test)]
mod tests {
    use super::{Exclusion, is_excluded};

    fn exclusions<const N: usize>(items: [&str; N]) -> Vec<Exclusion> {
        items.iter().map(|s| Exclusion::new(s)).collect()
    }

    #[test]
    fn exact_match_excludes_the_path() {
        assert!(is_excluded("/stream", &exclusions(["/stream"])));
    }

    #[test]
    fn children_of_a_prefix_are_excluded() {
        assert!(is_excluded("/stream/1", &exclusions(["/stream"])));
    }

    #[test]
    fn sibling_paths_are_not_excluded() {
        assert!(!is_excluded("/streaming", &exclusions(["/stream"])));
    }

    #[test]
    fn empty_list_excludes_nothing() {
        assert!(!is_excluded("/stream", &exclusions([])));
    }

    #[test]
    fn regex_entry_matches_the_full_path() {
        let entries = exclusions(["regex:^/files/.+\\.zip$"]);
        assert!(is_excluded("/files/a/b.zip", &entries));
        assert!(!is_excluded("/files/a/b.tar", &entries));
        assert!(!is_excluded("/other/files/a.zip", &entries));
    }

    #[test]
    fn regex_entries_coexist_with_prefix_entries() {
        let entries = exclusions(["/stream", "regex:^/assets/.+\\.(png|jpg)$"]);
        assert!(is_excluded("/stream/1", &entries));
        assert!(is_excluded("/assets/logo.png", &entries));
        assert!(!is_excluded("/assets/logo.svg", &entries));
    }

    #[test]
    fn regex_dots_do_not_act_as_wildcards_in_prefix_entries() {
        assert!(!is_excluded("/api/v1X0/users", &exclusions(["/api/v1.0"])));
    }

    #[test]
    #[should_panic(expected = "invalid exclusion regex")]
    fn invalid_regex_fails_fast() {
        Exclusion::new("regex:^/broken(");
    }
}
