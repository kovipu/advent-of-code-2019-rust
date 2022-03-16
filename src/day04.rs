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
    let n0 = nth_digit(password, 0).unwrap();
    let n1 = nth_digit(password, 1).unwrap();
    let n2 = nth_digit(password, 2).unwrap();
    let n3 = nth_digit(password, 3).unwrap();
    let n4 = nth_digit(password, 4).unwrap();
    let n5 = nth_digit(password, 5).unwrap();

    // Ugliest code ever
    is_increasing(password)
        && (n0 == n1 || n1 == n2 || n2 == n3 || n3 == n4 || n4 == n5)
}

pub fn part_2(input: &str) -> i32 {
    let (start, end) = input.split_once("-").unwrap();

    let start = start.parse::<i32>().unwrap();
    let end = end.parse::<i32>().unwrap();

    let mut count = 0;

    for i in start..=end {
        if is_valid_2(i) {
            count += 1;
        }
    }

    count
}

fn is_valid_2(password: i32) -> bool {
    // there's exactly two same numbers in a row.
    // take slices of 4 -> if middle 2 are the same, the edges are not the same -> the whole thing is valid

    let mut has_exact_double = false;

    if !is_increasing(password) {
        return false;
    }

    for i in 0..5 {
        let n0 = if i == 0 {
            -1
        } else {
            nth_digit(password, i - 1).unwrap()
        };
        let n1 = nth_digit(password, i).unwrap();
        let n2 = nth_digit(password, i + 1).unwrap();
        let n3 = nth_digit(password, i + 2).unwrap_or(-1);

        if n0 != n1 && n1 == n2 && n2 != n3 {
            has_exact_double = true;
            break;
        }
    }

    return has_exact_double;
}

fn nth_digit(password: i32, n: usize) -> Option<i32> {
    Some(password.to_string().chars().nth(n)?.to_digit(10).unwrap() as i32)
}

fn is_increasing(password: i32) -> bool {
    let n0 = nth_digit(password, 0).unwrap();
    let n1 = nth_digit(password, 1).unwrap();
    let n2 = nth_digit(password, 2).unwrap();
    let n3 = nth_digit(password, 3).unwrap();
    let n4 = nth_digit(password, 4).unwrap();
    let n5 = nth_digit(password, 5).unwrap();

    // Ugliest code ever
    n0 <= n1
        && n1 <= n2
        && n2 <= n3
        && n3 <= n4
        && n4 <= n5
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
    fn test_is_valid_3() {
        assert_eq!(is_valid_2(112233), true);
    }

    #[test]
    fn test_is_valid_4() {
        assert_eq!(is_valid_2(123444), false);
    }

    #[test]
    fn test_is_valid_5() {
        assert_eq!(is_valid_2(111122), true);
    }
}
