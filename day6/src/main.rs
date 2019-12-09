use std::collections::HashMap;

fn main() {
    let input = aoc::day6_input();
    let mut count = 0;
    let orbits: HashMap<_, _> = input
        .iter()
        .map(|l| l.split(')'))
        .map(|mut parts| (parts.next().unwrap(), parts.next().unwrap()))
        .map(|(x, y)| (y, x))
        .collect();

    for mut body in orbits.keys() {
        while let Some(parent) = orbits.get(body) {
            count += 1;
            body = parent;
        }
    }

    println!("Part 1 / Count: {}", count);

    let you = parents(&orbits, "YOU");
    let santa = parents(&orbits, "SAN");
    let common = you.iter().position(|b| santa.contains(b)).unwrap();
    let count = common + santa.iter().position(|&b| b == you[common]).unwrap() + 2;

    println!("Part 2 / Count: {}", count);
}

fn parents<'a>(orbits: &'a HashMap<&str, &str>, body: &str) -> Vec<&'a str> {
    let mut body = orbits.get(body).unwrap();
    let mut chain = Vec::new();

    while let Some(parent) = orbits.get(body) {
        chain.push(*parent);
        body = parent;
    }

    chain
}
