use url::{ Url };

use crate::{ GitReference, IntoUrl };
use crate::{ Error, Result };

/// Information for referncing a specific git repository
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GitRepository {
    pub repo: Url,
    pub reference: GitReference,
    pub precise: Option<String>,
}


impl GitRepository {
    pub fn from_url_string(url: String) -> Result<Self> {
        let mut url = url.into_url()?;
        let mut reference = GitReference::Branch("master".to_string());
        for (k, v) in url.query_pairs() {
            match &k[..] {
                // Map older 'ref' to branch.
                "branch" | "ref" => reference = GitReference::Branch(v.into_owned()),

                "rev" => reference = GitReference::Rev(v.into_owned()),
                "tag" => reference = GitReference::Tag(v.into_owned()),
                _ => {}
            }
        }
        let precise = url.fragment().map(|s| s.to_owned());
        url.set_fragment(None);
        url.set_query(None);
        canonicalize_url(&url)
            .map(|url| {
                GitRepository {
                    repo: url,
                    reference,
                    precise,
                }
            })
    }
}

// Some hacks and heuristics for making equivalent URLs hash the same.
pub fn canonicalize_url(url: &Url) -> Result<Url> {
    let mut url = url.clone();

    // cannot-be-a-base-urls (e.g., `github.com:rust-lang-nursery/rustfmt.git`)
    // are not supported.
    if url.cannot_be_a_base() {
        return Err(Error::GitBaseUrlNotSupported(url));
    }

    // Strip a trailing slash.
    if url.path().ends_with('/') {
        url.path_segments_mut().unwrap().pop_if_empty();
    }

    // HACK: for GitHub URLs specifically, just lower-case
    // everything. GitHub treats both the same, but they hash
    // differently, and we're gonna be hashing them. This wants a more
    // general solution, and also we're almost certainly not using the
    // same case conversion rules that GitHub does. (See issue #84.)
    if url.host_str() == Some("github.com") {
        url.set_scheme("https").unwrap();
        let path = url.path().to_lowercase();
        url.set_path(&path);
    }

    // Repos can generally be accessed with or without `.git` extension.
    let needs_chopping = url.path().ends_with(".git");
    if needs_chopping {
        let last = {
            let last = url.path_segments().unwrap().next_back().unwrap();
            last[..last.len() - 4].to_owned()
        };
        url.path_segments_mut().unwrap().pop().push(&last);
    }

    Ok(url)
}