use std::collections::hash_map::RandomState;
use std::collections::hash_set::Intersection;
use std::collections::{HashMap, HashSet};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_maps_points() {
        let input = vec!["R8", "U5", "L5", "D3"]
            .into_iter()
            .map(String::from)
            .collect();
        let points = get_points(&input);
        println!("Points: {:#?}", points);
    }

    #[test]
    fn it_validates_example_1() {
        let path_a = get_points(
            &vec!["R75", "D30", "R83", "U83", "L12", "D49", "R71", "U7", "L72"]
                .into_iter()
                .map(String::from)
                .collect(),
        );

        let path_b = get_points(
            &vec!["U62", "R66", "U55", "R34", "D71", "R55", "D58", "R83"]
                .into_iter()
                .map(String::from)
                .collect(),
        );

        let a: HashSet<(isize, isize)> = path_a.keys().cloned().collect();
        let b: HashSet<(isize, isize)> = path_b.keys().cloned().collect();
        assert_eq!(closest(a.intersection(&b)), 159);
    }

    #[test]
    fn it_validates_example_2() {
        let path_a = get_points(
            &vec![
                "R98", "U47", "R26", "D63", "R33", "U87", "L62", "D20", "R33", "U53", "R51",
            ]
            .into_iter()
            .map(String::from)
            .collect(),
        );
        let path_b = get_points(
            &vec![
                "U98", "R91", "D20", "R16", "D67", "R40", "U7", "R15", "U6", "R7",
            ]
            .into_iter()
            .map(String::from)
            .collect(),
        );
        let a: HashSet<(isize, isize)> = path_a.keys().cloned().collect();
        let b: HashSet<(isize, isize)> = path_b.keys().cloned().collect();
        assert_eq!(closest(a.intersection(&b)), 135);
    }

    #[test]
    fn it_validates_example_2_1() {
        let path_a = get_points(
            &"R75,D30,R83,U83,L12,D49,R71,U7,L72"
                .split(",")
                .map(String::from)
                .collect(),
        );

        let path_b = get_points(
            &"U62,R66,U55,R34,D71,R55,D58,R83"
                .split(",")
                .map(String::from)
                .collect(),
        );
        let a: HashSet<(isize, isize)> = path_a.keys().cloned().collect();
        let b: HashSet<(isize, isize)> = path_b.keys().cloned().collect();
        assert_eq!(shortest(a.intersection(&b), &path_a, &path_b), 610);
    }

    #[test]
    fn it_validates_example_2_2() {
        let path_a = get_points(
            &"R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51"
                .split(",")
                .map(String::from)
                .collect(),
        );
        let path_b = get_points(
            &"U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
                .split(",")
                .map(String::from)
                .collect(),
        );
        let a: HashSet<(isize, isize)> = path_a.keys().cloned().collect();
        let b: HashSet<(isize, isize)> = path_b.keys().cloned().collect();
        assert_eq!(shortest(a.intersection(&b), &path_a, &path_b), 410);
    }
}

fn main() {
    let input = aoc::day3_input();
    let path_a = get_points(&input.get(0).unwrap());
    let path_b = get_points(&input.get(1).unwrap());
    let a: HashSet<(isize, isize)> = path_a.keys().cloned().collect();
    let b: HashSet<(isize, isize)> = path_b.keys().cloned().collect();

    let both = a.intersection(&b);
    println!("3.1: {}", closest(both.clone()));

    println!("3.2: {}", shortest(both, &path_a, &path_b))
}

fn closest(both: Intersection<(isize, isize), RandomState>) -> isize {
    both.map(|(x, y)| x.abs() + y.abs()).min().unwrap().clone()
}

fn shortest(
    both: Intersection<(isize, isize), RandomState>,
    a_line: &HashMap<(isize, isize), usize>,
    b_line: &HashMap<(isize, isize), usize>,
) -> usize {
    both.map(|key| a_line[key] + b_line[key])
        .min()
        .unwrap()
        .clone()
}

fn build_dx() -> HashMap<char, isize> {
    vec![('L', -1), ('R', 1), ('U', 0), ('D', 0)]
        .into_iter()
        .collect()
}

fn build_dy() -> HashMap<char, isize> {
    vec![('L', 0), ('R', 0), ('U', 1), ('D', -1)]
        .into_iter()
        .collect()
}

fn get_points(line: &Vec<String>) -> HashMap<(isize, isize), usize> {
    let dx = build_dx();
    let dy = build_dy();
    let mut set = HashMap::new();
    let mut x = 0;
    let mut y = 0;
    let mut length = 0;
    for mut step in line.iter().map(|a| a.chars()) {
        let d = step.next().unwrap();
        let n: usize = step.as_str().parse().unwrap();
        for _ in 0..n {
            x += dx[&d];
            y += dy[&d];
            length += 1;
            set.entry((x, y)).or_insert(length);
        }
    }
    set
}
