use crate::intcode;

pub fn part_1(input: &str) -> i32 {
    let mut program = intcode::parse(input);

    program[1] = 12;
    program[2] = 2;
    intcode::execute(&mut program, &[]);

    program[0]
}

pub fn part_2(input: &str) -> i32 {
    let original = intcode::parse(input);

    for noun in 0..100 {
        for verb in 0..100 {
            let mut program = original.clone();
            program[1] = noun;
            program[2] = verb;
            intcode::execute(&mut program, &[]);

            if program[0] == 19690720 {
                return 100 * noun + verb;
            }
        }
    }

    panic!("No solution found");
}
