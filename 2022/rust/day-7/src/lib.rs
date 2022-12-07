use std::path::PathBuf;

pub fn parse_input(input: &str) {
    let mut cwd: Vec<&str> = vec![];

    // delete directory result if it exists
    if std::path::Path::new("result").exists() {
        std::fs::remove_dir_all("result").unwrap();
    }

    // create a directory called result
    std::fs::create_dir("result").unwrap();

    let _ = input
        .lines()
        .map(|l| {
            if l.starts_with("$") {
                // split on whitespace
                let mut parts = l.split_whitespace();
                let _ = parts.next(); // skip the $

                let command = parts.next().unwrap();

                match command {
                    "cd" => {
                        let path = parts.next().unwrap();
                        if path.starts_with("/") {
                            cwd = vec!["/"];
                        }

                        let path_parts = path.split("/");

                        for p in path_parts {
                            match p {
                                ".." => {
                                    cwd.pop();
                                }
                                "." => {}
                                "" => {}
                                _ => {
                                    cwd.push(p);
                                }
                            }
                        }
                    }
                    _ => { /* do nothing */ }
                }
            }

            if l.chars().nth(0).unwrap().is_numeric() {
                let mut parts = l.split_whitespace();
                let size = parts.next().unwrap().parse::<usize>().unwrap();
                let name = parts.next().unwrap().to_string();

                let file =
                    std::fs::File::create(format!("result/{}/{}", cwd.join("/"), name)).unwrap();
                file.set_len(size as u64).unwrap();
            } else if l.starts_with("dir") {
                let mut parts = l.split_whitespace();
                let _ = parts.next(); // skip the dir
                let name = parts.next().unwrap().to_string();

                std::fs::create_dir(format!("result/{}/{}", cwd.join("/"), name)).unwrap();
            }
        })
        .collect::<()>();
}

pub fn get_dir_size(path: &PathBuf) -> usize {
    let mut total_size = 0;
    for entry in std::fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            total_size += path.metadata().unwrap().len() as usize;
        } else if path.is_dir() {
            total_size += get_dir_size(&path);
        }
    }

    total_size
}

pub fn get_dir_size_max(path: &PathBuf, max: usize) -> usize {
    let mut total_size = 0;

    // if the current directory is smaller than max, add it to the total
    let size = get_dir_size(path);
    if size <= max {
        total_size += size;
    }

    // check subdirectories
    for entry in std::fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            total_size += get_dir_size_max(&path, max);
        }
    }

    total_size
}

pub fn find_directories_with_min_size(path: &PathBuf, min: usize) -> Vec<PathBuf> {
    let mut dirs = vec![];

    // if the current directory is smaller than max, add it to the total
    let size = get_dir_size(path);
    if size >= min {
        dirs.push(path.clone());
    }

    // check subdirectories
    for entry in std::fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            let res = find_directories_with_min_size(&path, min);
            dirs.extend(res);
        }
    }

    dirs
}

pub fn process_part1(input: &str) -> String {
    // will create the result directory with actual files
    parse_input(input);

    // find all of the directories with a total size of at most 100000, then calculate the sum of their total sizes.
    let total = get_dir_size_max(&PathBuf::from("result"), 100000);

    total.to_string()
}

pub fn process_part2(input: &str) -> String {
    let update_size: usize = 30000000;

    let fs_size: usize = 70000000;
    let in_use: usize = get_dir_size(&PathBuf::from("result"));
    let available = fs_size - in_use;
    let fs_free_required = update_size - available;

    dbg!(fs_size);
    dbg!(in_use);
    dbg!(available);
    dbg!(fs_free_required);

    // will create the result directory with actual files
    parse_input(input);

    let dirs = find_directories_with_min_size(&PathBuf::from("result"), fs_free_required);

    // find the smallest directory in dirs
    let mut smallest_dir = dirs[0].clone();
    let mut smallest_size = get_dir_size(&smallest_dir);
    for dir in dirs {
        let size = get_dir_size(&dir);
        if size < smallest_size {
            smallest_dir = dir;
            smallest_size = size;
        }
    }

    smallest_size.to_string()
}

#[cfg(test)]
mod tests {

    use crate::*;

    const INPUT: &str = "$ cd /
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
7214296 k";

    #[test]
    #[ignore]
    pub fn test_part_1() {
        assert_eq!(process_part1(INPUT), "95437");
    }

    #[test]
    pub fn test_part_2() {
        assert_eq!(process_part2(INPUT), "24933642");
    }
}
