//! Embed the latest git commit into your code at compile time.
//!
//! Runs the `git` binary directly, so `git` must be available in `PATH`.

use std::ffi::OsStr;
use std::path::Path;
use std::process::Command;

use proc_macro::{TokenStream, TokenTree};
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

const DEFAULT_FALLBACK: &str = "unknown";

/// Expands to the hash of the latest commit, e.g. `b4db6c3`.
///
/// An optional `fallback` expression is used when git is unavailable,
/// defaulting to `"unknown"`.
///
/// ```
/// const GIT_VERSION: &str = gitver::git_version!();
/// const GIT_VERSION_FALLBACK: &str = gitver::git_version!(fallback = "dev");
/// ```
#[proc_macro]
pub fn git_version(input: TokenStream) -> TokenStream {
    let fallback = match parse_fallback(input) {
        Ok(fallback) => fallback,
        Err(err) => return quote!(compile_error!(#err)).into(),
    };

    let version = match latest_commit() {
        Some(commit) => quote!(#commit),
        None => fallback,
    };
    let dependency = git_state_dependency();
    quote!({
        #dependency
        #version
    })
    .into()
}

/// Parses `fallback = <expr>`, defaulting to `"unknown"`.
fn parse_fallback(input: TokenStream) -> Result<TokenStream2, String> {
    let mut tokens: Vec<TokenTree> = input.into_iter().collect();
    if tokens.is_empty() {
        return Ok(quote!(#DEFAULT_FALLBACK));
    }
    if matches!(tokens.last(), Some(TokenTree::Punct(punct)) if punct.as_char() == ',') {
        tokens.pop();
    }

    let mut tokens = tokens.into_iter();
    match tokens.next() {
        Some(TokenTree::Ident(name)) if name.to_string() == "fallback" => {}
        Some(other) => {
            return Err(format!("unexpected argument `{other}`, expected `fallback = \"...\"`"));
        }
        None => return Err("expected `fallback = \"...\"`".to_owned()),
    }
    match tokens.next() {
        Some(TokenTree::Punct(punct)) if punct.as_char() == '=' => {}
        _ => return Err("expected `=` after `fallback`".to_owned()),
    }

    let value: TokenStream = tokens.collect();
    if value.is_empty() {
        return Err("expected a value after `fallback =`".to_owned());
    }
    Ok(value.into())
}

/// Returns the hash of the latest commit.
fn latest_commit() -> Option<String> {
    let manifest_dir = std::env::var_os("CARGO_MANIFEST_DIR")?;
    run_git(&manifest_dir, &["log", "-1", "--format=%h"])
}

fn run_git(dir: &OsStr, args: &[&str]) -> Option<String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(dir)
        .args(args)
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    Some(String::from_utf8(output.stdout).ok()?.trim().to_owned())
}

/// Adds a build dependency on the git reflog so new commits trigger a rebuild.
fn git_state_dependency() -> Option<TokenStream2> {
    let manifest_dir = std::env::var_os("CARGO_MANIFEST_DIR")?;
    let path = Path::new(&manifest_dir)
        .join(".git")
        .join("logs")
        .join("HEAD")
        .canonicalize()
        .ok()?;
    let path = path.to_str()?;
    Some(quote!(include_bytes!(#path);))
}
