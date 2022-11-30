use std::collections::HashMap;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct Cave {
    label: String,
    is_big_cave: bool,
}

struct Caves {
    label_to_cave: HashMap<String, Cave>,
}

struct Edges {
    cave_to_neighbours: HashMap<Cave, Vec<Cave>>,
}

pub struct CaveSystem {
    edges: Edges,
}

impl CaveSystem {
    pub fn from_input(input: &str) -> CaveSystem {
        // let mut cave_system: CaveSystem<'b> = CaveSystem::new();

        let mut cave_to_neighbours = HashMap::new();

        for (label_1, label_2) in input
            .split('\n')
            .map(|s| s.split('-'))
            .map(|mut split| (split.next().unwrap(), split.next().unwrap()))
        {
            let cave_1 = Cave {
                label: String::from(label_1),
                is_big_cave: label_1 == label_1.to_uppercase(),
            };
            let cave_2 = Cave {
                label: String::from(label_2),
                is_big_cave: label_2 == label_2.to_uppercase(),
            };

            if !cave_to_neighbours.contains_key(&cave_1) {
                cave_to_neighbours.insert(cave_1.clone(), vec![cave_2.clone()]);
            } else {
                cave_to_neighbours
                    .get_mut(&cave_1)
                    .unwrap()
                    .push(cave_2.clone());
            }
            if !cave_to_neighbours.contains_key(&cave_2) {
                cave_to_neighbours.insert(cave_2, vec![cave_1.clone()]);
            } else {
                cave_to_neighbours.get_mut(&cave_2).unwrap().push(cave_1);
            }
        }

        CaveSystem {
            edges: Edges { cave_to_neighbours },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_label_map(cave_system: &CaveSystem) -> HashMap<String, Vec<String>> {
        cave_system
            .edges
            .cave_to_neighbours
            .iter()
            .map(|(k, v)| {
                (
                    k.label.clone(),
                    v.iter().map(|x| x.label.clone()).collect::<Vec<_>>(),
                )
            })
            .collect()
    }

    #[test]
    fn test_parse() {
        let input = r"a-b
a-c";
        let cave_system = CaveSystem::from_input(input);

        let label_map = get_label_map(&cave_system);

        let cave_to_neighbours = cave_system.edges.cave_to_neighbours;

        assert_eq!(cave_to_neighbours.len(), 3);

        assert_eq!(label_map["a"], vec!["b", "c"]);
        assert_eq!(label_map["b"], vec!["a"]);
        assert_eq!(label_map["c"], vec!["a"]);
    }
}
