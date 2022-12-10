use std::borrow::Borrow;
use std::cell::{RefCell, Ref};
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Write, Display};
use std::fs;
use std::rc::{Rc, Weak};

use itertools::Itertools;

#[derive(Debug)]
struct Command {
    arguments: Vec<String>,
    output: String,
}

#[derive(Debug)]
struct ShellExecution {
    commands: Vec<Command>,
}

#[derive(Debug)]
enum LsOutputItem<'a> {
    File(&'a str, usize),
    Directory(&'a str),
}

#[derive(Debug)]
struct LsOutput<'a>(Vec<LsOutputItem<'a>>);

impl<'a> TryFrom<&'a str> for LsOutput<'a> {
    type Error = &'static str;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let mut entries = Vec::new();
        for line in value.lines() {
            let mut split = line.split(" ");
            if let Ok(number) = split
                .next()
                .ok_or("No size.")?
                .parse::<usize>()
                .map_err(|_| "Size isn't a number.")
            {
                // A file entry.
                let name = split.next().ok_or("No name.")?;
                entries.push(LsOutputItem::File(name, number));
            } else {
                // A directory entry.
                let dir_name = split.next().ok_or("No name.")?;
                entries.push(LsOutputItem::Directory(dir_name));
            }
        }
        Ok(LsOutput(entries))
    }
}

impl TryFrom<&str> for ShellExecution {
    type Error = &'static str;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let mut result = ShellExecution {
            commands: Vec::new(),
        };
        for line in input.lines() {
            let parts_and_spaces = line.chars().group_by(|char| *char == ' ');
            let mut parts = parts_and_spaces
                .into_iter()
                .filter_map(|(is_space, group)| {
                    if is_space {
                        None
                    } else {
                        Some(group.collect::<String>())
                    }
                });
            if let Some(part) = parts.nth(0) {
                if part == "$" {
                    // A command.
                    result.commands.push(Command {
                        arguments: Vec::new(),
                        output: String::new(),
                    });
                    for part in parts {
                        result.commands.last_mut().unwrap().arguments.push(part);
                    }
                } else {
                    // A command's output.
                    let output = &mut result
                        .commands
                        .last_mut()
                        .ok_or("The first line needs to be a command.")?
                        .output;
                    writeln!(output, "{line}").map_err(|_| "IO error.")?;
                }
            }
        }
        Ok(result)
    }
}

#[derive(Debug)]
enum DirectoryEntry {
    File(String, usize),
    Directory(Rc<RefCell<Directory>>),
}

impl DirectoryEntry {
    fn name(&self) -> String {
        match self {
            DirectoryEntry::File(name, _) => name.to_string(),
            DirectoryEntry::Directory(directory) => RefCell::borrow(&directory).name.to_string(),
        }
    }
}

impl Display for DirectoryEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DirectoryEntry::File(name, size) => write!(f, "- {} (file, size={})", name, size).unwrap(),
            DirectoryEntry::Directory(directory) => write!(f, "{}", RefCell::borrow(directory)).unwrap(),
        }
        Ok(())
    }
}

#[derive(Debug)]
struct Directory {
    parent: Weak<RefCell<Directory>>,
    name: String,
    content: HashMap<String, DirectoryEntry>,
}

impl Directory {
    fn add_entry(&mut self, entry: DirectoryEntry) {
        self.content.insert(entry.name().to_string(), entry);
    }

    /// The filesize of only the files in the current directory.
    fn shallow_filesize(&self) -> usize {
        let mut total_size = 0;
        for entry in self.content.values() {
            if let DirectoryEntry::File(_, size) = entry {
                total_size += size;
            }
        }
        total_size
    }

    fn total_size(&self) -> usize {
        let mut total_size = 0;
        for entry in self.content.values() {
            match entry {
                DirectoryEntry::File(_, size) => {
                    total_size += size;
                }
                DirectoryEntry::Directory(directory) => {
                    total_size += RefCell::borrow(directory).total_size();
                }
            }
        }
        total_size
    }
}

impl Display for Directory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "- {} (dir)", self.name).unwrap();
        for entry in self.content.values().into_iter().collect_vec()[..self.content.len()-1].iter() {
            writeln!(f, "\t{}", entry).unwrap();
        }
        if let Some(item)= self.content.values().last() {
            write!(f, "\t{}", item).unwrap();
        }
        Ok(())
    }
}

fn total_filesize_smaller_than(directory: Rc<RefCell<Directory>>, max_size: usize) -> usize {
    let mut total_size = 0;
    let directory = RefCell::borrow(&directory);
    let toplevel_shallow_size = directory.total_size();
    if toplevel_shallow_size <= max_size {
        total_size += toplevel_shallow_size;
    }
    for entry in directory.content.values() {
        if let DirectoryEntry::Directory(directory) = entry {
            total_size += total_filesize_smaller_than(Rc::clone(&directory), max_size);
        }
    }
    total_size
}

fn smallest_to_delete(directory: Rc<RefCell<Directory>>, minimum_to_free: usize) -> Option<Rc<RefCell<Directory>>> {
    let mut smallest_to_delete_m: Option<Rc<RefCell<Directory>>> = None;
    for entry in RefCell::borrow(&directory).content.values() {
        if let DirectoryEntry::Directory(directory) = entry {
            if RefCell::borrow(directory).total_size() >= minimum_to_free {
                if let Some(ref other) = smallest_to_delete_m {
                    if RefCell::borrow(&other).total_size() > RefCell::borrow(directory).total_size() {
                        smallest_to_delete_m = Some(Rc::clone(&directory));
                    }
                } else {
                    smallest_to_delete_m = Some(Rc::clone(&directory));
                }
            }
            if let Some(other) = smallest_to_delete(Rc::clone(directory), minimum_to_free) {
                smallest_to_delete_m = Some(other);
            }
        }
    }
    smallest_to_delete_m
}

fn main() -> Result<(), Box<dyn Error>> {
    // 1. Parse the input as valid utf-8.
    let input = fs::read_to_string("data/day7.txt")?;

    // 2. Parse the utf-8 input into a shell execution history.
    // Shell execution history:
    //  - Commands
    //  - Output
    let shell_execution = ShellExecution::try_from(input.as_str())?;

    let root_directory = Rc::new_cyclic(|weak| {
        RefCell::new(Directory {
            parent: weak.clone(),
            name: String::from("/"),
            content: HashMap::new(),
        })
    });


    let mut current_directory = Rc::clone(&root_directory);
    // We assume that there is always an ls before a cd, so the filesystem has
    // all the necessary info to cd.
    for command in shell_execution.commands[1..].iter() {
        match command.arguments[0].as_str() {
            "ls" => {
                // Add all the entries to the current directory.
                let ls_entries = LsOutput::try_from(command.output.as_str())?;
                for entry in ls_entries.0 {
                    match entry {
                        LsOutputItem::File(name, size) => {
                            // Add the file to the current directory.
                            current_directory.borrow_mut().add_entry(DirectoryEntry::File(name.to_string(), size));
                        }
                        LsOutputItem::Directory(name) => {
                            // Add the directory to the current directory.
                            current_directory.borrow_mut().add_entry(DirectoryEntry::Directory(Rc::new(RefCell::new(Directory { parent: Rc::downgrade(&Rc::clone(&current_directory)), name: name.to_string(), content: HashMap::new() }))))
                        }
                    }
                }
            }
            "cd" => {
                // Change `current_directory`.
                if command.arguments[1] == ".." {
                    let current_directory_borrowed = RefCell::borrow(&current_directory);
                    let new_directory = Rc::clone(&current_directory_borrowed.parent.upgrade().ok_or("Not upgradable!")?);
                    drop(current_directory_borrowed);
                    current_directory = new_directory;
                } else if command.arguments[1] == "/" {
                    current_directory = Rc::clone(&root_directory);
                } else {
                    let new_directory;
                    let current_directory_borrowed: Ref<_> = RefCell::borrow(&current_directory);
                    if let DirectoryEntry::Directory(directory) = current_directory_borrowed.content.get(&command.arguments[1]).ok_or("No entry with that name.")? {
                        new_directory = Rc::clone(&directory);
                    } else {
                        panic!("Can't change into non-existant directory.");
                    }
                    drop(current_directory_borrowed);
                    current_directory = new_directory;
                }
            }
            _ => {
                panic!("Not a valid command.");
            }
        }
    }

    let free = 70000000 - RefCell::borrow(&root_directory).total_size();
    let needed = 30000000 - free;
    println!("{}", RefCell::borrow(&root_directory));
    println!("{}", total_filesize_smaller_than(Rc::clone(&root_directory), 100000));
    // Ok this doesn't work. I admit it. I borrowed someone elses work. Sue me.
    // This one really sucked major ass. Might come back once my mind can handle
    // seeing the word filesystem again.
    println!("{}", RefCell::borrow(&smallest_to_delete(Rc::clone(&root_directory), needed).unwrap()).total_size());

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_given_input() -> Result<(), Box<dyn Error>> {
        let given = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k".to_string();


    // 2. Parse the utf-8 input into a shell execution history.
    // Shell execution history:
    //  - Commands
    //  - Output
    let shell_execution = ShellExecution::try_from(given.as_str())?;

    let root_directory = Rc::new_cyclic(|weak| {
        RefCell::new(Directory {
            parent: weak.clone(),
            name: String::from("/"),
            content: HashMap::new(),
        })
    });


    let mut current_directory = Rc::clone(&root_directory);
    // We assume that there is always an ls before a cd, so the filesystem has
    // all the necessary info to cd.
    for command in shell_execution.commands[1..].iter() {
        match command.arguments[0].as_str() {
            "ls" => {
                // Add all the entries to the current directory.
                let ls_entries = LsOutput::try_from(command.output.as_str())?;
                for entry in ls_entries.0 {
                    match entry {
                        LsOutputItem::File(name, size) => {
                            // Add the file to the current directory.
                            current_directory.borrow_mut().add_entry(DirectoryEntry::File(name.to_string(), size));
                        }
                        LsOutputItem::Directory(name) => {
                            // Add the directory to the current directory.
                            current_directory.borrow_mut().add_entry(DirectoryEntry::Directory(Rc::new(RefCell::new(Directory { parent: Rc::downgrade(&Rc::clone(&current_directory)), name: name.to_string(), content: HashMap::new() }))))
                        }
                    }
                }
            }
            "cd" => {
                // Change `current_directory`.
                if command.arguments[1] == ".." {
                    let current_directory_borrowed = RefCell::borrow(&current_directory);
                    let new_directory = Rc::clone(&current_directory_borrowed.parent.upgrade().ok_or("Not upgradable!")?);
                    drop(current_directory_borrowed);
                    current_directory = new_directory;
                } else if command.arguments[1] == "/" {
                    current_directory = Rc::clone(&root_directory);
                } else {
                    let new_directory;
                    let current_directory_borrowed: Ref<_> = RefCell::borrow(&current_directory);
                    if let DirectoryEntry::Directory(directory) = current_directory_borrowed.content.get(&command.arguments[1]).ok_or("No entry with that name.")? {
                        new_directory = Rc::clone(&directory);
                    } else {
                        panic!("Can't change into non-existant directory.");
                    }
                    drop(current_directory_borrowed);
                    current_directory = new_directory;
                }
            }
            _ => {
                panic!("Not a valid command.");
            }
        }
    }
    println!("{}", RefCell::borrow(&root_directory));

    assert_eq!(total_filesize_smaller_than(root_directory, 100_000), 95_437);
    Ok(())

    }
}
