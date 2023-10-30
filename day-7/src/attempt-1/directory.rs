use std::collections::BTreeMap;
use crate::system::{File, State};

#[derive(Clone)]
pub struct Directory {
  name: String,
  pub parent: Option<Box<Directory>>,
  pub children: BTreeMap<String, Directory>,
  files: BTreeMap<String, File>,
} 

impl Directory {
  pub fn new(name: String, parent: Option<Box<Directory>>, children: BTreeMap<String, Directory>, files: BTreeMap<String, File>) -> Directory {
    Directory {name, parent, children, files}
  }

  pub fn create(name: String, parent: Option<Box<Directory>>) -> Directory {
    Directory { name, parent, children: BTreeMap::new(), files: BTreeMap::new() }
  }

  pub fn default() -> Directory {
    Directory { name: String::from("System"), parent: None, children: BTreeMap::new(), files: BTreeMap::new() }
  }

  pub fn search_children(&self, name: String) -> Option<&Directory> {
    self.children.get(&name)
  }

  pub fn back_dir(&self) -> State {
    let dir = match &self.parent {
      Some(x) => x,
      None => panic!("No parent directory"),
    };
    State {dir: *dir.to_owned()}
  }

  pub fn get_name(&self) -> &str {
    &self.name
  }

  pub fn new_child(&mut self, name: String) {
    let dir = Directory::create(name.clone(), Some(Box::new(self.clone())));
    self.children.insert(name, dir);
  }
  
  pub fn new_file(&mut self, size: usize, name: String) {
    let file = File::create(name.clone(), size, self.clone());
    self.files.insert(name, file);
  }

  pub fn print_children(&self) -> String {
    let mut output = String::from("\"");
    for i in &self.children {
      output += i.1.get_name();
      output += ", "
    }
    output += "\"";
    output
  }

  pub fn print_parent(&self) -> String {
    let parent = self.parent.as_deref();
    let output = match parent {
      Some(x) => x.get_name(),
      None => "No Parent",
    };
    output.to_string()
  }
}
