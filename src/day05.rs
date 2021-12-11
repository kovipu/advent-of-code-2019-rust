use crate::intcode;

pub fn part_1(input: &str) -> i32 {
    let mut program = intcode::parse(input);
    let output = intcode::execute(&mut program, &[1]);
    *output.last().unwrap()
}

pub fn part_2(input: &str) -> i32 {
    let mut program = intcode::parse(input);
    let output = intcode::execute(&mut program, &[5]);
    *output.last().unwrap()
}
