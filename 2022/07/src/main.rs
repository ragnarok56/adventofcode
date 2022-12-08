use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::{PathBuf};
use std::collections::{HashMap};

#[derive(Debug)]
struct Dir {
    files: Vec<u32>
}

impl Dir {
    fn total_size(&self) -> u32 {
        self.files.iter().sum()
    }
}

#[derive(Debug)]
struct Session {
    pwd: PathBuf,
    dirs: HashMap<String, Dir>
}

impl Session {
    fn parse_input(&mut self, path: &str) -> () {
        let file = File::open(path).expect("file doesnt exist");

        let reader = BufReader::new(file);

        let lines = reader.lines().map(|x| x.unwrap());

        for line in lines {
            let mut line_split = line.split(" ");
            let first = line_split.next().unwrap();
            match first {
                "$" => match line_split.next().unwrap() {
                    "cd" => self.parse_cd(&line),
                    "ls" => (),
                    _ => ()
                },
                _ => ()
            }

            self.parse_ls_line(&line);
        }
    }
    
    fn parse_ls_line(&mut self, line: &str) {
        let mut line_split = line.split(" ");
        let filesize = line_split.next().unwrap().parse::<u32>();
        if filesize.is_ok() {
            let pwd = self.pwd.to_str().unwrap();
            if self.dirs.contains_key(pwd) {
                let file_list = &mut self.dirs.get_mut(pwd).unwrap().files;
                file_list.push(filesize.unwrap())
            }
        }
    }

    fn parse_cd(&mut self, line: &str) {
        let cd_dir = line.split(" ").nth(2).unwrap();
        match cd_dir {
            ".." => {
                self.pwd.pop();
            },
            "/" => {
                let mut path_buf = PathBuf::new();
                path_buf.push("/");
                self.pwd = path_buf;
            },
            _ => {
                self.pwd.push(cd_dir);
            }
        };

        let pwd = self.pwd.to_str().unwrap();
        if !self.dirs.contains_key(pwd) {
            self.dirs.insert(pwd.to_string(), Dir { files: Vec::new() });
        }
    }
}


fn main() {
    let mut path_buf = PathBuf::new();
    path_buf.push("/");

    let mut session = Session { 
        pwd: path_buf,
        dirs: HashMap::new()
    };
    session.parse_input("in");

    let all_dirs: Vec<String> = session.dirs.keys().map(|x| x.clone()).collect();
    let mut summed_dirs: HashMap<String, u32> = HashMap::new();

    for d in all_dirs.iter() {
        
        let all_child_dirs: u32 = all_dirs
            .iter()
            .filter(|x| x.starts_with(d) && *x != d)
            .map(|x| session.dirs[x].total_size())
            .sum();

        summed_dirs.insert(d.to_string(), session.dirs[d].total_size() + all_child_dirs);
    }

    let total: u32 = summed_dirs.values().filter(|x| *x <= &100000).sum();

    println!("{}", total);

    let space_needed = 30000000 - (70000000 - summed_dirs["/"]);
    let mut potential_dirs_to_delete_sizes: Vec<u32> = summed_dirs.values().filter(|x| *x > &space_needed).map(|x| *x).collect();
    potential_dirs_to_delete_sizes.sort();
    let total = potential_dirs_to_delete_sizes.iter().next().unwrap();
    println!("{}", total);
}