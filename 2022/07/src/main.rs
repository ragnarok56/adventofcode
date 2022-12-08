use std::fs::File;
use std::hash::Hash;
use std::io::{BufReader, BufRead};
use std::path::{Path, PathBuf};
use std::collections::{HashMap};


#[derive(Debug)]
struct Fil {
    name: String,
    size: u32
}

#[derive(Debug)]
struct Dir {
    name: String,
    dirs: HashMap<String, Dir>,
    files: HashMap<String, Fil>
}

#[derive(Debug)]
struct Dir2 {
    files: Vec<Fil>
}

impl Dir2 {
    fn total_size(&self) -> u32 {
        self.files.iter().map(|x| x.size).sum()
    }
}

impl Dir {
    fn new(name: String) -> Dir {
        Dir {
            name: name,
            dirs: HashMap::new(),
            files: HashMap::new()
        }
    }
}

#[derive(Debug)]
struct Session {
    pwd: PathBuf,
    cwd: Dir,
    dirs: HashMap<String, Dir2>
}

#[derive(PartialEq)]
enum ParseState {
    CD,
    LS, 
    NONE
}

impl Session {
    fn parse_input(&mut self, path: &str) -> () {
        let file = File::open(path).expect("file doesnt exist");

        let reader = BufReader::new(file);

        let mut lines = reader.lines().map(|x| x.unwrap());

        let mut parse_state = ParseState::NONE;

        for line in lines {
            let mut line_split = line.split(" ");
            let first = line_split.next().unwrap();
            match first {
                "$" => match line_split.next().unwrap() {
                    "cd" => self.parse_cd(&line),
                    "ls" => {
                        parse_state = ParseState::LS;
                    }
                    _ => ()
                },
                _ => ()
            }

            if parse_state == ParseState::LS {
                self.parse_ls_line(&line)
            }
        }
    }
    
    fn parse_ls_line(&mut self, line: &str) {
        let mut line_split = line.split(" ");
        let dir_or_filesize = line_split.next().unwrap();
        match dir_or_filesize {
            "dir" => {
                // let dir_name = line_split.next().unwrap().to_string();
                // let cwd_dirs = &mut self.cwd.dirs;
                // if !cwd_dirs.contains_key(&dir_name) {
                //     cwd_dirs.insert(dir_name.clone(), Dir::new(dir_name));
                // }
            },
            "$" => (),
            _ => {
                let pwd = self.pwd.to_str().unwrap();
                if self.dirs.contains_key(pwd) {
                    let file = Fil { 
                        name: line_split.next().unwrap().to_string(),
                        size: dir_or_filesize.parse().unwrap()
                    };
                    let file_list = &mut self.dirs.get_mut(pwd).unwrap().files;
                    file_list.push(file)
                }
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
            self.dirs.insert(pwd.to_string(), Dir2 { files: Vec::new() });
        }
    }
}


fn part1() -> () {
    let mut path_buf = PathBuf::new();
    path_buf.push("/");

    let fs = Dir::new("/".to_string());

    let mut session = Session { 
        cwd: fs,
        pwd: path_buf,
        dirs: HashMap::new()
    };
    session.parse_input("in");

    for d in session.dirs.keys() {
        println!("{}: {:?}", d, session.dirs[d].files);
    }

    let mut all_dirs: Vec<_> = session.dirs.keys().collect();
    all_dirs.sort_by(|a, b| {
        a.split('/').count().cmp(&b.split('/').count())
    });
    all_dirs.reverse();
    let mut summed_dirs: HashMap<String, u32> = HashMap::new();

    for d in all_dirs.iter() {
        println!("Handling {}", d);
        let all_child_dirs: u32 = all_dirs.iter()
            .filter(|x| x.starts_with(*d) && *x != d)
            .map(|x| {
                print!("{},", x);
                session.dirs[*x].total_size()
            })
            .sum();
        println!("Total Size: {}, Child Dir Size: {}", session.dirs[*d].total_size(), all_child_dirs);
        summed_dirs.insert(d.to_string(), session.dirs[*d].total_size() + all_child_dirs);
    }


    // attempt to start at bottom and work way up to find totals, this 
    // did not work like, at all
    // for d in all_dirs {
    //     // println!("{}: {:?}", d, session.dirs[d].total_size());
    //     if summed_dirs.contains_key(d) {
    //         println!("Already visited {} from child directory", d);
    //         continue
    //     }
    //     let mut path_buf = PathBuf::new();
    //     path_buf.push(d);
    //     for dir in path_buf.ancestors() {
    //         let dirname = dir.to_str().unwrap();
    //         let size = session.dirs[dirname].total_size();
    //         println!("Adding {}: {:?}", dirname, size);
    //         if !summed_dirs.contains_key(dirname) {
    //             summed_dirs.insert(dirname.to_string(), size);
    //         } else {
    //             summed_dirs.insert(dirname.to_string(), summed_dirs[dirname] + size);
    //         }
    //     }
    // }

    println!("Cumulative Totals");
    for d in summed_dirs.keys() {
        println!("{}: {:?}", d, summed_dirs[d]);
    }

    let total: u32 = summed_dirs.values().filter(|x| *x <= &100000).sum();

    println!("{}", total);

    let space_needed = 30000000 - (70000000 - summed_dirs["/"]);
    let mut potential_dirs_to_delete_sizes: Vec<u32> = summed_dirs.values().filter(|x| *x > &space_needed).map(|x| *x).collect();
    potential_dirs_to_delete_sizes.sort();
    let total = potential_dirs_to_delete_sizes.iter().next().unwrap();
    println!("{}", total);
}


fn main() {
    part1();
}