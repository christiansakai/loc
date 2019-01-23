use std::env;
use std::env::Args;
use std::io;
use std::io::Read;
use std::fs::File;
use std::path::Path;
use std::collections::VecDeque;

fn main() {

    // let file_paths = depth_first_search_file("../provantagex");
    // let total_loc = file_paths.iter()
    //     .fold(0, |acc, file_path| {
    //         if let Ok(count) = count_loc(file_path) {
    //             return acc + count;
    //         }

    //         acc
    //     });

    // println!("Total Lines of Code {:?}", total_loc);
}

struct Data {
    root_path: String,
    ignores: Vec<String>
}

impl Data {
    fn new(args: Args) -> Result<Data, String> {
        if let Some(root_path) = env::args()
            .skip(1)
            .find(|string| {
                !string.starts_with("--b")
            }) {
            return Err(String::from("No root path provided"));
        }

        let mut ignores = Vec::new();

        // let ignore_path = env::args()
        //     .skip(1)
        //     .find(|string| {
        //         string.starts_with("--b")
        //     });

        // if let Ok(mut file) = File::open(ignore_path) {
        //     let mut contents = String::new();
        //     if let Ok(_) = file.read_to_string(&mut contents) {
        //         ignores = contents
        //             .trim()
        //             .split('\n')
        //             .collect();
        //     }
        // }


        Ok(Data {
            root_path,
            ignores,
        })
    }
}

fn depth_first_search_file(root_path: &str) -> Vec<String> {
    let mut file_paths: Vec<String> = Vec::new();
    let mut deque: VecDeque<String> = VecDeque::new();
    deque.push_back(String::from(root_path));

    while !deque.is_empty() {
        let path_opt = deque.pop_front();
        if let Some(path_str) = path_opt {
            let path = Path::new(&path_str);
            if path.is_dir() {
                for entry in path.read_dir().expect("read_dir call failed") {
                    if let Ok(entry) = entry {
                        if let Some(path_str) = entry.path().to_str() {
                            let path_str = String::from(path_str);
                            deque.push_front(path_str);
                        }
                    }
                }

            } else if path.is_file() {
                file_paths.push(path_str.clone());
            }
        }
    }

    file_paths
}

fn count_loc(file_path: &str) -> io::Result<usize> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let lines: Vec<&str> = contents
        .trim()
        .split('\n')
        .collect();

    Ok(lines.len())
}
