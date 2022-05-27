pub mod document;
pub mod keys;
pub mod link;
pub mod profile;
pub mod workspace;

use crate::parse::MdFile;
use std::path::PathBuf;

pub struct Relation<M> {
    pub id: uuid::Uuid,
    pub relative: M,
}

pub struct Association<M> {
    pub id: uuid::Uuid,
    pub relative: M,
}

pub struct Property {
    pub id: uuid::Uuid,
    pub intrinsic: bool,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Tag {
    pub name: String,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Line(usize);

impl std::ops::Deref for Line {
    type Target = usize;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct Location {
    ln: Line,
    col: usize,
    offset: usize,
}
impl Location {
    pub fn start() -> Location {
        Location {
            ln: Line(0),
            col: 0,
            offset: 0,
        }
    }
}

pub trait Element: PartialEq + Clone + std::fmt::Debug {
    type Id;
    type Parent;

    fn id(self) -> Self::Id;

    fn parent(self) -> Option<Self::Parent>;

    fn element_name() -> String;

    fn name(self) -> String;

    fn file(self) -> PathBuf;

    fn loc(self) -> Location;
}
