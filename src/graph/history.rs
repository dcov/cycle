use sha2;
use std::collections::HashMap;

pub type Hash = [u8; 32];

pub struct Id {
    commit: Hash,
    index: u16,
}

pub type Patch = Box<[u8]>;

pub struct Change {
    node: Id,
    patch: Patch,
}

pub struct Edge {
    from: Id,
    to: Id,
    namespace: Box<str>,
    namespace_id: u16,
}

pub struct Commit {
    // The nodes' ids are simply the id of a commit, paired with its index in the vec.
    // This is made posssible/easier due to commits being immutable, so the only crucial steps are reading and writing
    // commits to disk.
    nodes_added: Box<[Patch]>,
    // Similar to nodes_added
    edges_added: Box<[Edge]>,

    nodes_removed: Box<[Id]>,
    edges_removed: Box<[Id]>,

    nodes_changed: Box<[Change]>,
}

type CommitMap = HashMap<Hash, Commit>;

pub struct History(CommitMap);

impl History {
    pub fn new() -> Self {
        Self(CommitMap::new())
    }
}

#[cfg(test)]
mod tests {}
