use crate::{ GitReference };

/// 
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Source {
    /// A git repository
    Git {
        reference: GitReference,
        /// For example, the exact Git revision of the specified branch for a Git Source.
        precise: Option<String>,
    },
    /// A local path..
    Path,
    /// A remote registry.
    RemoteRegistry,
    /// A local filesystem-based registry.
    LocalRegistry,
    /// A directory-based registry.
    Directory,
}