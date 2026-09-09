//! Segment-aware path-prefix matching shared by the middlewares' per-path
//! exclusions: `/stream` excludes `/stream` and `/stream/1` but not
//! `/streaming`. A bare `/` excludes everything.

/// Returns whether `path` matches any of the configured prefixes
pub(super) fn is_excluded(path: &str, prefixes: &[String]) -> bool {
    prefixes.iter().any(|prefix| {
        let prefix = prefix.trim_end_matches('/');
        path == prefix || path.starts_with(&format!("{prefix}/"))
    })
}

#[cfg(test)]
mod tests {
    use super::is_excluded;

    fn prefixes<const N: usize>(items: [&str; N]) -> Vec<String> {
        items.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn exact_match_excludes_the_path() {
        assert!(is_excluded("/stream", &prefixes(["/stream"])));
    }

    #[test]
    fn children_of_a_prefix_are_excluded() {
        assert!(is_excluded("/stream/1", &prefixes(["/stream"])));
    }

    #[test]
    fn sibling_paths_are_not_excluded() {
        assert!(!is_excluded("/streaming", &prefixes(["/stream"])));
    }

    #[test]
    fn empty_list_excludes_nothing() {
        assert!(!is_excluded("/stream", &prefixes([])));
    }
}
