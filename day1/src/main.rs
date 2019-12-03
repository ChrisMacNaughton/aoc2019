#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_mass_12() {
        let input = 12;
        assert_eq!(calculate_fuel(input), 2);
    }

    #[test]
    fn validate_mass_14() {
        let input = 14;
        assert_eq!(calculate_fuel(input), 2);
    }

    #[test]
    fn validate_mass_1969() {
        let input = 1969;
        assert_eq!(calculate_fuel(input), 966);
    }

    #[test]
    fn validate_mass_100756() {
        let input = 100756;
        assert_eq!(calculate_fuel(input), 50346);
    }
}

fn main() {
    println!(
        "OUTPUT: {:#?}",
        aoc::day1_input()
            .iter()
            .map(|i| calculate_fuel(*i))
            .sum::<usize>()
    );
}

fn calculate_fuel(mut input: usize) -> usize {
    let mut total = 0;
    while input > 0 {
        input = calculate_unit(input);
        total += input;
    }
    total
}

fn calculate_unit(input: usize) -> usize {
    let new_base = (input as f64 / 3.0).floor() as usize;
    match new_base.overflowing_sub(2) {
        (parts, false) => parts,
        (_, true) => 0,
    }
}
