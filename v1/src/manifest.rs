use crate::Workspace;

#[derive(Clone, Debug, PartialEq)]
pub enum Manifest {
    Workspace (Workspace),
    Project,
}