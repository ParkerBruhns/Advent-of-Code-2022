#[allow(unused_imports)]
use std::collections::BTreeMap;
use crate::Directory;

// mod directory;

#[allow(dead_code)]
#[derive(Clone)]
pub struct File {
  name: String,
  size: usize,
  parent: Directory,
}

impl File {
  pub fn create(name: String, size: usize, parent: Directory) -> File {
    File {name, size, parent}
  }
}

pub struct State {
  pub dir: Directory,
}
impl State {

}
