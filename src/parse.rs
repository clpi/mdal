use super::types::{Association, Element, Line, Location, Property, Relation};
use indexmap::IndexMap;
use pulldown_cmark::{
    BrokenLink, CowStr, Event, LinkDef, LinkType, OffsetIter, Options, Parser, Tag,
};
use serde::{Deserialize, Serialize};
use std::{
    fmt::{Debug, Display},
    iter::Peekable,
    ops::{Deref, DerefMut, Range},
    path::{Path, PathBuf},
};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct FileId(usize);

impl Deref for FileId {
    type Target = usize;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<usize> for FileId {
    fn from(u: usize) -> Self {
        Self(u)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Item {
    pub id: uuid::Uuid,
    pub name: String,
    pub associations: Vec<usize>,
    pub relations: Vec<usize>,
    pub properties: Vec<usize>,
}
impl Item {}

#[derive(Debug)]
pub struct MdFile {
    name: String,
    headers: IndexMap<u8, IndexMap<Line, String>>,
    list_items: IndexMap<Line, String>,
    todo_items: IndexMap<Line, String>,
    raw: String,
    dir: PathBuf,
}
