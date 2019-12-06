use std::collections::HashMap;

const INPUT: std::ops::RangeInclusive<usize> = 246515..=739105;
const RADIX: u32 = 10;

#[cfg(test)]
mod tests {
    use super::*;

    // now invalid on 4.2
    // #[test]
    // fn it_tests_example_1() {
    //     let input = 111111;
    //     assert_eq!(valid(&input), true);
    // }

    #[test]
    fn it_tests_example_2() {
        let input = 223450;
        assert_eq!(valid(&input), false);
    }

    #[test]
    fn it_tests_example_3() {
        let input = 123789;
        assert_eq!(valid(&input), false);
    }

    #[test]
    fn it_tests_example_4() {
        let input = 112233;
        assert_eq!(valid(&input), true);
    }

    #[test]
    fn it_tests_example_6() {
        let input = 123444;
        assert_eq!(valid(&input), false);
    }

    #[test]
    fn it_tests_example_5() {
        let input = 111122;
        assert_eq!(valid(&input), true);
    }
}

fn valid(input: &usize) -> bool {
    let mut last_num = 0;
    let mut digits = HashMap::new();
    for c in input
        .to_string()
        .as_str()
        .chars()
        .map(|i| i.to_digit(RADIX))
    {
        match c {
            Some(c) => {
                let v = digits.entry(c).or_insert(0);
                *v += 1;
                if c >= last_num {
                    last_num = c;
                } else {
                    return false;
                }
            }
            None => continue,
        }
    }

    digits.iter().find(|(_k, v)| **v == 2).is_some()
}

fn main() {
    println!("Valid options: {}", INPUT.filter(|i| valid(i)).count());
}
