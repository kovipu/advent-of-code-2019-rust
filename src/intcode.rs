pub fn execute(program: &mut Vec<i32>) {
    let mut pointer: usize = 0;

    loop {
        let opcode = program[pointer];

        match opcode {
            1 => {
                let read_addr_a = program[program[pointer + 1] as usize];
                let read_addr_b = program[program[pointer + 2] as usize];
                let write_addr = program[pointer + 3] as usize;

                program[write_addr] = read_addr_a + read_addr_b;

                pointer += 4;
            }
            2 => {
                let read_addr_a = program[program[pointer + 1] as usize];
                let read_addr_b = program[program[pointer + 2] as usize];
                let write_addr = program[pointer + 3] as usize;

                program[write_addr] = read_addr_a * read_addr_b;

                pointer += 4;
            }
            99 => break,
            _ => panic!("Unknown opcode: {}", opcode),
        }
    }
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
        execute(&mut program);
        assert_eq!(program, vec![2, 0, 0, 0, 99]);
    }

    #[test]
    fn test_multiplication() {
        let mut program = vec![2, 3, 0, 3, 99];
        execute(&mut program);
        assert_eq!(program, vec![2, 3, 0, 6, 99]);
    }

    #[test]
    fn test_multiplication_with_larger_numbers() {
        let mut program = vec![2, 4, 4, 5, 99, 0];
        execute(&mut program);
        assert_eq!(program, vec![2, 4, 4, 5, 99, 9801]);
    }

    #[test]
    fn test_multiple_operations() {
        let mut program = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        execute(&mut program);
        assert_eq!(program, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }
}
