//! Transport-layer middleware, one submodule per concern.
//!
//! - [`request_id`]: propagate the request id into tracing spans and response headers
//! - [`timeout`]: enforce a request time budget, with per-path exclusions
//! - [`compression`]: compress responses, with per-path exclusions
//!
//! The middlewares share [`is_excluded`] for their per-path exclusion matching.

pub mod compression;
pub mod request_id;
pub mod timeout;

/// Segment-aware prefix match shared by the per-path exclusions: `/stream`
/// excludes `/stream` and `/stream/1` but not `/streaming`. A bare `/` excludes
/// everything.
pub(crate) fn is_excluded(path: &str, prefixes: &[String]) -> bool {
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
