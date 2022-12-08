use FileCommand::*;

#[derive(Debug)]
struct Folder {
    name:    String,
    folders: Vec<Folder>,
    files:   Vec<File>,
}

impl Folder {
    fn new(name: String) -> Self {
        return Folder { name, folders: vec!(), files: vec!() }
    }
    fn get_by_name(&mut self, name: &str) -> Option<&mut Folder> {
        for folder in &mut self.folders {
            if folder.name == name {
                return Some(folder)
            }
        }
        return None
    }
    fn get_size(&self) -> u32 {
        let mut size = 0;
        for file in &self.files {
            size += file.size
        }
        for folder in &self.folders {
            size += folder.get_size()
        }
        return size
    }
    fn traverse(&self) -> Vec<u32> {
        let mut new_sizes = vec!(self.get_size());
        for folder in &self.folders {
            new_sizes.append(&mut folder.traverse());
        }
        return new_sizes
    }
}

#[derive(Debug)]
struct File {
    name: String,
    size: u32,
}

impl File {
    fn new(name: String, size: u32) -> Self {
        return File { name, size }
    }
}

#[derive(Debug)]
struct FileSystem {
    root: Folder,
    cur_path: Vec<String>,
}

impl FileSystem {
    fn new() -> Self {
        return FileSystem { root: Folder::new("".to_string()), cur_path: vec!() }
    }
    fn cd(&mut self, path_name: String) {
        if path_name == "..".to_string() {
            self.cur_path.remove(self.cur_path.len() - 1);
        } else if path_name == "/".to_string() {
            self.cur_path = vec!()
        } else {
            self.cur_path.push(path_name);
        }
    }
    fn get_folder_at_path(&mut self) -> &mut Folder {
        let mut folder = &mut self.root;
        if self.cur_path.len() == 0 {
            return folder
        }
        for folder_name in &self.cur_path {
            folder = folder.get_by_name(&folder_name).expect(format!("No folder {}", folder_name).as_str());
        }
        folder
    }
    fn path_to_string(&self, delim: &str) -> String {
        let mut string = String::new();
        for path_part in &self.cur_path {
            string.push_str(delim);
            string.push_str(&path_part);
        }
        return string
    }
    fn add_folder(&mut self, name: String) {
        let new_folder = Folder::new(name);
        self.get_folder_at_path().folders.push(new_folder)
    }    
    fn add_file(&mut self, name: String, size: u32) {
        let new_file = File::new(name, size);
        self.get_folder_at_path().files.push(new_file);
    }
    fn traverse(&self) -> Vec<u32> {
        let sizes = self.root.traverse();
        return sizes
    }
}

struct Parser {
    input: String,
    index: usize,
}

impl Iterator for Parser {
    type Item = FileCommand;

    fn next(&mut self) -> Option<Self::Item> {
        let command_vec: Vec<&str> = self.input.trim().split('\n').nth(self.index)?.split(' ').collect();
        self.index += 1;
        match command_vec[0] {
            "$" => { 
                if command_vec[1] == "cd" {
                    return Some(ChangeDir(command_vec[2].to_string()))
                } else if command_vec[1] == "ls" {
                    return Some(ListDir())
                } else {
                    unreachable!()
                }
            } 
            "dir" => { return Some(AddFolder(command_vec[1].to_string())) }
            num if num.parse::<u32>().is_ok() => { return Some(AddFile(command_vec[1].to_string(), num.parse::<u32>().unwrap())) }
            _ => { eprintln!("unknown command line {}", self.index); panic!() }
        }
    }
}

enum FileCommand {
    ChangeDir(String),
    ListDir(),
    AddFile(String, u32),
    AddFolder(String),
}

pub fn calc_solution(input: String) -> (String, String) {
    let mut file_system = FileSystem::new();
    let parser = Parser{ input, index: 0 };
    for command in parser {
        match command {
            ChangeDir(dir) => {
                file_system.cd(dir);
            }
            ListDir() => {}
            AddFolder(dir) => {
                file_system.add_folder(dir)
            }
            AddFile(name, size) => {
                file_system.add_file(name, size)
            }
        }
    }
    let folder_sizes = file_system.traverse();
    let free_space = 70000000 - file_system.root.get_size();
    
    let size_sum: u32 = folder_sizes.iter().filter(|size| size < &&(100000 as u32)).sum();
    let size_biggest: u32 = *folder_sizes.iter().filter(|size| size > &&(30000000 - free_space as u32)).min().unwrap();
    
    return (size_sum.to_string(), size_biggest.to_string())
}
