use std::fs;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

fn read_file(filename: &str) -> String {
    fs::read_to_string(filename).expect("Something went wrong reading the file")
}

pub fn day1_input() -> Vec<usize> {
    let input = read_file("../data/day1.txt");
    input
        .trim()
        .split('\n')
        .map(|i| i.parse().unwrap())
        .collect()
}

pub fn day2_input() -> Vec<usize> {
    let input = read_file("../data/day2.txt");
    input
        .trim()
        .split(',')
        .map(|i| i.parse().unwrap())
        .collect()
}
