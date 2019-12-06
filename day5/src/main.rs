use intcode::Intcode;

fn main() {
    let input = aoc::day5_input();
    let mut intcode = Intcode::new(input.clone());
    intcode.run();
}
