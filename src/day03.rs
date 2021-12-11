use std::collections::HashSet;

pub fn part_1(input: &str) -> i32 {
    let (first, second) = input.split_once('\n').unwrap();
    let first = wire(first);
    let second = wire(second);

    intersection(&first, &second)
        .iter()
        .map(manhattan_distance)
        .min()
        .unwrap()
}

pub fn part_2(_input: &str) -> i32 {
    0
}

fn wire(path: &str) -> Vec<(i32, i32)> {
    let path = path.split(',');
    let mut points: Vec<(i32, i32)> = vec![];

    for segment in path {
        let (direction, distance) = segment.split_at(1);
        let distance = distance.parse::<i32>().unwrap();

        let current_point = points.last().unwrap_or(&(0, 0));
        let x = current_point.0;
        let y = current_point.1;

        match direction {
            "R" => {
                for i in 1..=distance {
                    points.push((x + i, y));
                }
            }
            "L" => {
                for i in 1..=distance {
                    points.push((x - i, y));
                }
            }
            "U" => {
                for i in 1..=distance {
                    points.push((x, y + i));
                }
            }
            "D" => {
                for i in 1..=distance {
                    points.push((x, y - i));
                }
            }
            _ => panic!("Unknown direction"),
        }
    }

    points
}

fn intersection(a: &[(i32, i32)], b: &[(i32, i32)]) -> Vec<(i32, i32)> {
    // Convert to sets for O(1) lookup
    let a: HashSet<String> = a.iter().map(|(x, y)| format!("{},{}", x, y)).collect();
    let b: HashSet<String> = b.iter().map(|(x, y)| format!("{},{}", x, y)).collect();

    a.intersection(&b)
        .map(|s| {
            let (x, y) = s.split_once(',').unwrap();
            (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap())
        })
        .collect()
}

fn manhattan_distance(point: &(i32, i32)) -> i32 {
    point.0.abs() + point.1.abs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 159);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(""), 0);
    }
}

#[cfg(test)]
const INPUT: &str = "R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83";
