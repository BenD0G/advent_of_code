mod input;

use std::{collections::HashMap, path::PathBuf};

use input::INPUT;

fn make_graph() -> HashMap<String, u64> {
    let mut dir_path_to_size = HashMap::new();
    dir_path_to_size.insert("/".into(), 0);
    let mut current_path = PathBuf::new();
    for line in INPUT.lines() {
        if line == "$ cd /" {
            current_path = PathBuf::from("/");
        } else if line == "$ cd .." {
            current_path = current_path.parent().unwrap().into();
        } else if line.starts_with("$ cd ") {
            current_path = current_path.join(&line[5..])
        } else if line.starts_with("dir") || line.starts_with("$ ls") {
            continue;
        } else {
            let size = line.split(' ').next().unwrap().parse::<u64>().unwrap();
            let mut path_to_update = current_path.clone();
            loop {
                // println!("{:?} {:?}", path_to_update, dir_path_to_size);
                if !dir_path_to_size.contains_key(path_to_update.to_str().unwrap()) {
                    dir_path_to_size.insert(path_to_update.to_str().unwrap().to_string(), 0);
                }
                *dir_path_to_size
                    .get_mut(path_to_update.to_str().unwrap())
                    .unwrap() += size;
                path_to_update = match path_to_update.parent() {
                    Some(x) => x.into(),
                    None => break,
                }
            }
        }
    }
    dir_path_to_size
}

fn solve_1(graph: &HashMap<String, u64>) -> u64 {
    graph.values().filter(|x| **x <= 100000).sum()
}

fn solve_2(graph: &HashMap<String, u64>) -> u64 {
    let free_space = 70_000_000 - graph["/"];
    let required_space = 30_000_000 - free_space;
    let mut values = graph
        .values()
        .filter(|x| **x >= required_space)
        .collect::<Vec<_>>();
    values.sort();
    *values[0]
}

fn main() {
    let graph = make_graph();
    println!("{}", solve_1(&graph));
    println!("{}", solve_2(&graph));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        assert_eq!(&"$ cd foo"[5..], "foo")
    }
}
