use intcode::Intcode;

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
    let input = aoc::day2_input();
    let mut intcode = Intcode::new(input.clone());
    intcode.set(1, 12);
    intcode.set(2, 2);
    intcode.run();
    println!("{}", intcode.get(0));

    println!("Day 2.2");
    println!();
    let (noun, verb) = find_output(&input, 19690720);
    println!("{}{}", noun, verb)
}
