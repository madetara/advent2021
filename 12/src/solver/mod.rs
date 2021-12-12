use std::collections::{HashMap, HashSet};

pub fn solve_1(data: &HashMap<String, Vec<String>>) -> usize {
    let mut visited: HashSet<String> = HashSet::new();
    let start = String::from("start");
    visited.insert(start.clone());

    find_paths(data, &mut visited, &start)
}

pub fn solve_2(data: &HashMap<String, Vec<String>>) -> usize {
    let mut visited = HashMap::new();
    let start = String::from("start");
    visited.insert(start.clone(), 1);

    find_paths_2(data, &mut visited, &start, false)
}

fn find_paths(
    vertices: &HashMap<String, Vec<String>>,
    visited_vertices: &mut HashSet<String>,
    current_vertex: &String,
) -> usize {
    let mut result = 0;
    for vertex in vertices.get(current_vertex).unwrap() {
        if vertex == "end" {
            result += 1;
            continue;
        }

        if !visited_vertices.contains(vertex) {
            if vertex.chars().all(|c| c.is_lowercase()) {
                visited_vertices.insert(vertex.clone());
            }

            result += find_paths(vertices, visited_vertices, vertex);
            visited_vertices.remove(vertex);
        }
    }

    result
}

fn find_paths_2(
    vertices: &HashMap<String, Vec<String>>,
    visited_vertices: &mut HashMap<String, usize>,
    current_vertex: &String,
    has_double_visited_vertex: bool,
) -> usize {
    let mut result = 0;
    for vertex in vertices.get(current_vertex).unwrap() {
        if vertex == "end" {
            result += 1;
            continue;
        }

        if vertex == "start" {
            continue;
        }

        let entry = visited_vertices.entry(vertex.clone()).or_insert(0);

        if *entry == 0 || (*entry == 1 && !has_double_visited_vertex) {
            if vertex.chars().all(|c| c.is_lowercase()) {
                *entry += 1;
            }

            let double_visit = *entry >= 2;
            result += find_paths_2(
                vertices,
                visited_vertices,
                vertex,
                double_visit || has_double_visited_vertex,
            );

            let after_entry = visited_vertices.get_mut(vertex).unwrap();

            if *after_entry > 0 {
                *after_entry -= 1;
            }
        }
    }

    result
}
