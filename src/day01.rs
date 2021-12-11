pub fn part_1(input: &str) -> i32 {
    input
        .lines()
        .map(|line| calculate_fuel(line.parse::<i32>().unwrap()))
        .sum()
}

pub fn part_2(input: &str) -> i32 {
    input
        .lines()
        .map(|line| calculate_fuel_recursive(line.parse::<i32>().unwrap()))
        .sum()
}

fn calculate_fuel(mass: i32) -> i32 {
    mass / 3 - 2
}

fn calculate_fuel_recursive(mass: i32) -> i32 {
    let fuel = calculate_fuel(mass);
    if fuel <= 0 {
        return 0;
    }
    fuel + calculate_fuel_recursive(fuel)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_fuel() {
        assert_eq!(calculate_fuel(100756), 33583);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(calculate_fuel_recursive(100756), 50346);
    }
}
