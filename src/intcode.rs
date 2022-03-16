pub fn execute(program: &mut Vec<i32>, input: &[i32]) -> Vec<i32> {
    let mut input_pointer = 0;
    let mut output: Vec<i32> = vec![];

    let mut pointer: usize = 0;

    loop {
        let instruction = program[pointer];

        let opcode: i32 = instruction % 100;

        if opcode == 99 {
            break;
        }

        let immediate_mode_a: bool = instruction / 100 % 10 == 1;
        let immediate_mode_b: bool = instruction / 1000 % 10 == 1;

        let a: i32 = if immediate_mode_a {
            program[pointer + 1]
        } else {
            program[program[pointer + 1] as usize]
        };

        // handle special opcodes here
        if opcode == 3 {
            // input
            let value = input[input_pointer];
            input_pointer += 1;

            let write_addr = program[pointer + 1] as usize;
            program[write_addr] = value;
            pointer += 2;
            continue;
        } else if opcode == 4 {
            // output
            output.push(a);
            pointer += 2;
            continue;
        }

        let b: i32 = if immediate_mode_b {
            program[pointer + 2]
        } else {
            program[program[pointer + 2] as usize]
        };

        let c: usize = program[pointer + 3] as usize;

        match opcode {
            1 => {
                // addition
                program[c] = a + b;
                pointer += 4;
            }
            2 => {
                // multiplication
                program[c] = a * b;
                pointer += 4;
            }
            5 => {
                // jump-if-true
                if a != 0 {
                    pointer = b as usize;
                } else {
                    pointer += 3;
                }
            }
            6 => {
                // jump-if-false
                if a == 0 {
                    pointer = b as usize;
                } else {
                    pointer += 3;
                }
            }
            7 => {
                // less than
                if a < b {
                    program[c] = 1;
                } else {
                    program[c] = 0;
                }

                pointer += 4;
            }
            8 => {
                // equals
                if a == b {
                    program[c] = 1;
                } else {
                    program[c] = 0;
                }

                pointer += 4;
            }
            99 => {
                dbg!("HALT");
                break;
            }
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

    #[test]
    fn test_output() {
        let mut program = vec![4, 0, 99];
        let output = execute(&mut program, &[]);
        assert_eq!(output, vec![4]);
    }
}
