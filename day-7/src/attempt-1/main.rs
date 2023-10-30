use std::collections::HashMap;
use std::collections::BTreeMap;

#[allow(unused)]    
use system::{State, File};
use directory::Directory;

mod directory;
mod system;

fn main() {
    let path = r"src/input.txt";
    let input = std::fs::read_to_string(path).unwrap().lines().map(String::from).collect::<Vec<String>>();
    let mut state = State{dir: Directory::default()};

    for (i, line) in input.into_iter().enumerate() {
        println!("Line #: {}\nState: {}\nParent: {}\nChildren: {}", i+1, &state.dir.get_name(), state.dir.print_parent(), &state.dir.print_children());

        if line.starts_with("$") {
            state = command(line, state);
        } else {
            state = read(line, state);
        }
    }
    println!("\n\nFINISHED!");
}

fn read(line: String, mut state: State) -> State {
    //reads the line and puts in into a collection
    let mut components = line.split_whitespace();
    match components.next() {
        Some(x) => match x.eq("dir") {
            true => state.dir.new_child(components.next().unwrap().to_owned()),
            false => {
                let size = x.parse::<usize>().unwrap();
                let name = components.next().unwrap().to_owned();
                state.dir.new_file(size, name);
            },
        }
        None => panic!("No components"),
    }
    return state;
}

fn command(line: String, state: State) -> State {
    let components = line.split_whitespace().collect::<Vec<&str>>();
    if components[1].eq("cd") {
        if components[2].eq("..") {
            state.dir.back_dir();
            println!("\nCurrent Dir: {}\n", state.dir.get_name());
            return state;
        } else if components[2].eq("/") {
            return State { dir: Directory::new(String::from(components[2]), None, BTreeMap::new(), BTreeMap::new())};
        } else {
           let directory = state.dir.search_children(components[2].to_owned());
           let dir = match directory {
               Some(x) => x.to_owned(),
               None => state.dir,
           };
            return State{ dir };
            // todo!("cd with name")
        }
    } else if components[1].eq("ls") {
        return state;
    }
    panic!("No State Found");
}
