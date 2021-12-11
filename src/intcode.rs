pub fn execute(program: &mut Vec<i32>, input: &[i32]) -> Vec<i32> {
    let mut input_pointer = 0;
    let mut output: Vec<i32> = vec![];

    let mut pointer: usize = 0;

    loop {
        let instruction = program[pointer];

        let opcode: i32 = instruction % 100;
        let immediate_mode_a: bool = instruction / 100 % 10 == 1;
        let immediate_mode_b: bool = instruction / 1000 % 10 == 1;
        let _immediate_mode_c: bool = instruction / 10000 % 10 == 1;

        match opcode {
            1 => {
                // addition
                let a = if immediate_mode_a {
                    program[pointer + 1]
                } else {
                    program[program[pointer + 1] as usize]
                };

                let b = if immediate_mode_b {
                    program[pointer + 2]
                } else {
                    program[program[pointer + 2] as usize]
                };

                let write_addr = program[pointer + 3] as usize;

                program[write_addr] = a + b;

                pointer += 4;
            }
            2 => {
                // multiplication
                let a = if immediate_mode_a {
                    program[pointer + 1]
                } else {
                    program[program[pointer + 1] as usize]
                };

                let b = if immediate_mode_b {
                    program[pointer + 2]
                } else {
                    program[program[pointer + 2] as usize]
                };

                let write_addr = program[pointer + 3] as usize;

                program[write_addr] = a * b;

                pointer += 4;
            }
            3 => {
                // input
                let a = input[input_pointer];
                input_pointer += 1;

                let write_addr = program[pointer + 1] as usize;

                program[write_addr] = a;

                pointer += 2;
            }
            4 => {
                // output
                let a = if immediate_mode_a {
                    program[pointer + 1]
                } else {
                    program[program[pointer + 1] as usize]
                };

                output.push(a);

                pointer += 2;
            }
            5 => {
                // jump-if-true
                let a = if immediate_mode_a {
                    program[pointer + 1]
                } else {
                    program[program[pointer + 1] as usize]
                };

                let b = if immediate_mode_b {
                    program[pointer + 2]
                } else {
                    program[program[pointer + 2] as usize]
                };

                if a != 0 {
                    pointer = b as usize;
                } else {
                    pointer += 3;
                }
            }
            6 => {
                // jump-if-false
                let a = if immediate_mode_a {
                    program[pointer + 1]
                } else {
                    program[program[pointer + 1] as usize]
                };

                let b = if immediate_mode_b {
                    program[pointer + 2]
                } else {
                    program[program[pointer + 2] as usize]
                };

                if a == 0 {
                    pointer = b as usize;
                } else {
                    pointer += 3;
                }
            }
            7 => {
                // less than
                let a = if immediate_mode_a {
                    program[pointer + 1]
                } else {
                    program[program[pointer + 1] as usize]
                };

                let b = if immediate_mode_b {
                    program[pointer + 2]
                } else {
                    program[program[pointer + 2] as usize]
                };

                let write_addr = program[pointer + 3] as usize;

                if a < b {
                    program[write_addr] = 1;
                } else {
                    program[write_addr] = 0;
                }

                pointer += 4;
            }
            8 => {
                // equals
                let a = if immediate_mode_a {
                    program[pointer + 1]
                } else {
                    program[program[pointer + 1] as usize]
                };

                let b = if immediate_mode_b {
                    program[pointer + 2]
                } else {
                    program[program[pointer + 2] as usize]
                };

                let write_addr = program[pointer + 3] as usize;

                if a == b {
                    program[write_addr] = 1;
                } else {
                    program[write_addr] = 0;
                }

                pointer += 4;
            }
            99 => break,
            _ => panic!("Unknown opcode: {}", opcode),
        }
    }

    output
}

pub fn parse(input: &str) -> Vec<i32> {
    input
        .trim()
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addition() {
        let mut program = vec![1, 0, 0, 0, 99];
        execute(&mut program, &[]);
        assert_eq!(program, vec![2, 0, 0, 0, 99]);
    }

    #[test]
    fn test_multiplication() {
        let mut program = vec![2, 3, 0, 3, 99];
        execute(&mut program, &[]);
        assert_eq!(program, vec![2, 3, 0, 6, 99]);
    }

    #[test]
    fn test_multiplication_with_larger_numbers() {
        let mut program = vec![2, 4, 4, 5, 99, 0];
        execute(&mut program, &[]);
        assert_eq!(program, vec![2, 4, 4, 5, 99, 9801]);
    }

    #[test]
    fn test_multiple_operations() {
        let mut program = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        execute(&mut program, &[]);
        assert_eq!(program, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }

    #[test]
    fn test_immediate_mode() {
        let mut program = vec![1002, 4, 3, 4, 33];
        execute(&mut program, &[]);
        assert_eq!(program, vec![1002, 4, 3, 4, 99]);
    }

    #[test]
    fn test_immediate_mode_with_larger_numbers() {
        let mut program = vec![1101, 100, -1, 4, 0];
        execute(&mut program, &[]);
        assert_eq!(program, vec![1101, 100, -1, 4, 99]);
    }

    #[test]
    fn test_input() {
        let mut program = vec![3, 0, 99];
        execute(&mut program, &[42]);
        assert_eq!(program, vec![42, 0, 99]);
    }
}
