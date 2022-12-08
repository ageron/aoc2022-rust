use itertools::Itertools;
use std::collections::HashMap;

struct DirectoryContent {
    total_file_sizes: u64,
    subdir_names: Vec<String>,
}

fn total_dir_size(
    directory_contents: &HashMap<Vec<String>, DirectoryContent>,
    directory_path: &Vec<String>,
) -> u64 {
    let content = directory_contents.get(directory_path).unwrap();
    let total_subdirectory_sizes: u64 = content
        .subdir_names
        .iter()
        .map(|subdir_name| {
            let mut subdir_path = directory_path.clone();
            subdir_path.push(subdir_name.clone());
            total_dir_size(directory_contents, &subdir_path)
        })
        .sum();
    content.total_file_sizes + total_subdirectory_sizes
}

pub fn run(input: &str) {
    let commands = input.lines().collect_vec();
    let mut directory_contents: HashMap<Vec<String>, DirectoryContent> = HashMap::new();
    let mut current_path: Vec<String> = vec![]; // default to root
    let mut index = 0;
    while index < commands.len() {
        let command = commands[index];
        match command {
            "$ cd /" => {
                current_path.clear();
                index += 1;
            }
            "$ cd .." => {
                current_path.pop();
                index += 1;
            }
            "$ ls" => {
                let mut total_file_sizes = 0;
                let mut subdir_names = vec![];
                index += 1;
                while index < commands.len() {
                    let file = commands[index];
                    if file.starts_with("$ ") {
                        break;
                    }
                    let (dir_or_size, name) = file.split(' ').collect_tuple().unwrap();
                    if dir_or_size == "dir" {
                        subdir_names.push(name.to_string());
                    } else {
                        let file_size = dir_or_size.parse::<u64>().unwrap();
                        total_file_sizes += file_size;
                    }
                    index += 1;
                }
                directory_contents.insert(
                    current_path.clone(),
                    DirectoryContent {
                        total_file_sizes,
                        subdir_names,
                    },
                );
            }
            _ => {
                // $ cd {subdirectory name}
                let subdir_name = command.split(' ').last().unwrap().to_string();
                current_path.push(subdir_name);
                index += 1;
            }
        }
    }
    let directory_sizes = directory_contents
        .keys()
        .map(|directory_path| total_dir_size(&directory_contents, directory_path))
        .collect_vec();

    // part 1
    let total_size_of_small_directories: u64 = directory_sizes
        .iter()
        .filter(|&&size| size <= 100_000)
        .sum();
    println!("{}", total_size_of_small_directories);

    // part 2
    let total_disk_space = 70_000_000;
    let needed = 30_000_000;
    let available_size = total_disk_space - total_dir_size(&directory_contents, &vec![]);
    let minimum_to_delete = needed - available_size;
    let size_of_smallest_directory_to_delete = directory_sizes
        .iter()
        .filter(|&&size| size >= minimum_to_delete)
        .min()
        .unwrap();
    println!("{}", size_of_smallest_directory_to_delete);
}
