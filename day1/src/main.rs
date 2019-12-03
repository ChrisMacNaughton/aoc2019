const INPUT: [usize; 100] = [
    88623, 101095, 149899, 89383, 54755, 73496, 115697, 99839, 65903, 140201, 95734, 144728,
    113534, 82199, 98256, 107126, 54686, 61243, 54763, 119048, 58863, 134097, 84959, 130490,
    145813, 115794, 130398, 69864, 133973, 58382, 75635, 77153, 132645, 91661, 126536, 118977,
    137568, 100341, 142080, 63342, 84557, 51961, 61956, 87922, 92488, 107572, 51373, 70148, 80672,
    134880, 105721, 100138, 80394, 145117, 50567, 122606, 112408, 110737, 111521, 144309, 65761,
    113147, 58920, 96623, 65479, 66576, 94244, 64493, 142334, 65969, 99461, 143656, 134661, 90115,
    122994, 66994, 135658, 134336, 102958, 111410, 107930, 54711, 101397, 111350, 86453, 134383,
    134276, 130342, 80522, 64875, 68182, 83400, 121302, 105996, 123580, 130373, 123987, 107932,
    78930, 132068,
];

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
        INPUT.iter().map(|i| calculate_fuel(*i)).sum::<usize>()
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