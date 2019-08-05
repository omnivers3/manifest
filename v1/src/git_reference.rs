/// Information to find a specific commit in a Git repository.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GitReference {
    /// From a tag.
    Tag(String),
    /// From the HEAD of a branch.
    Branch(String),
    /// From a specific revision.
    Rev(String),
}