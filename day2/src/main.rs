use intcode::Intcode;

static INPUT: [usize; 149] = [
    1, 0, 0, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 1, 6, 19, 2, 19, 6, 23, 1, 23, 5, 27, 1, 9,
    27, 31, 1, 31, 10, 35, 2, 35, 9, 39, 1, 5, 39, 43, 2, 43, 9, 47, 1, 5, 47, 51, 2, 51, 13, 55,
    1, 55, 10, 59, 1, 59, 10, 63, 2, 9, 63, 67, 1, 67, 5, 71, 2, 13, 71, 75, 1, 75, 10, 79, 1, 79,
    6, 83, 2, 13, 83, 87, 1, 87, 6, 91, 1, 6, 91, 95, 1, 10, 95, 99, 2, 99, 6, 103, 1, 103, 5, 107,
    2, 6, 107, 111, 1, 10, 111, 115, 1, 115, 5, 119, 2, 6, 119, 123, 1, 123, 5, 127, 2, 127, 6,
    131, 1, 131, 5, 135, 1, 2, 135, 139, 1, 139, 13, 0, 99, 2, 0, 14, 0,
];

fn find_output(program: &[usize], desired: usize) -> (usize, usize) {
    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut int = Intcode::new(program.to_vec());
            int.set(1, noun);
            int.set(2, verb);
            int.run();
            if int.get(0) == desired {
                return (noun, verb);
            }
        }
    }

    unreachable!()
}

fn main() {
    let mut intcode = Intcode::new(INPUT.to_vec());
    intcode.set(1, 12);
    intcode.set(2, 2);
    intcode.run();
    println!("{}", intcode.get(0));

    println!("Day 2.2");
    println!();
    let (noun, verb) = find_output(&INPUT, 19690720);
    println!("{}{}", noun, verb)
}
