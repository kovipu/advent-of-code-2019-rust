pub fn part_1(input: &str) -> i32 {
    let (start, end) = input.split_once("-").unwrap();

    let start = start.parse::<i32>().unwrap();
    let end = end.parse::<i32>().unwrap();

    let mut count = 0;

    for i in start..=end {
        if is_valid(i) {
            count += 1;
        }
    }

    count
}

fn is_valid(password: i32) -> bool {
    let n0 = nth_digit(password, 0);
    let n1 = nth_digit(password, 1);
    let n2 = nth_digit(password, 2);
    let n3 = nth_digit(password, 3);
    let n4 = nth_digit(password, 4);
    let n5 = nth_digit(password, 5);

    // Ugliest code ever
    n0 <= n1
        && n1 <= n2
        && n2 <= n3
        && n3 <= n4
        && n4 <= n5
        && (n0 == n1 || n1 == n2 || n2 == n3 || n3 == n4 || n4 == n5)
}

fn nth_digit(password: i32, n: usize) -> i32 {
    password
        .to_string()
        .chars()
        .nth(n)
        .unwrap()
        .to_digit(10)
        .unwrap() as i32
}

pub fn part_2(_input: &str) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid() {
        assert_eq!(is_valid(111111), true);
    }

    #[test]
    fn test_is_valid_2() {
        assert_eq!(is_valid(223450), false);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(""), 0);
    }
}
