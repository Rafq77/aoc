fn parse_string(input: &str) -> Vec<(i64, i64)> {
    let mut lines = input.lines();
    lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect::<Vec<i64>>()
        .into_iter()
        .zip(
            lines
                .next()
                .unwrap()
                .split_whitespace()
                .skip(1)
                .map(|s| s.parse().unwrap())
                .collect::<Vec<i64>>(),
        )
        .collect()
}

fn parse_string_concatenated(input: &str) -> Vec<(i64, i64)> {
    let mut lines = input.lines();
    let time: i64 = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .collect::<String>()
        .parse()
        .unwrap();
    let distance: i64 = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .collect::<String>()
        .parse()
        .unwrap();

    vec![(time, distance)]
}

fn calc_optimized_square_solution(inputs: Vec<(i64, i64)>) -> i64 {
    let mut mults = Vec::<i64>::new();
    for round in inputs {
        let c = -round.1 as f64;
        let b = round.0 as f64;
        // let a // optimized away since its -1

        let d = b * b + 4. * c;
        let x_0 = (-b + d.sqrt()) / (-2.);
        let x_1 = (-b - d.sqrt()) / (-2.);

        let mut bonus = 0.;

        if x_0.ceil() == x_0 {
            bonus -= 1.;
        }
        if x_1.ceil() == x_1 {
            bonus -= 1.;
        }

        let result = (x_1.floor() - x_0.ceil() + 1. + bonus) as i64;

        mults.push(result);
    }

    mults.iter().product()
}

#[cfg(test)]
mod tests {
    use super::*;
    static TEST: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn parse_test() {
        assert_eq!(vec![(7, 9), (15, 40), (30, 200)], parse_string(TEST));
        assert_eq!(vec![(71530, 940200)], parse_string_concatenated(TEST));
    }

    #[test]
    fn full_test() {
        assert_eq!(288, calc_optimized_square_solution(parse_string(TEST)));
        assert_eq!(
            71503,
            calc_optimized_square_solution(parse_string_concatenated(TEST))
        );
    }
}

pub fn day06() {
    let _input = include_str!("input.txt");
    println!(
        "Day06 part1: {}",
        calc_optimized_square_solution(parse_string(_input))
    ); // 633080
    println!(
        "Day06 part2: {}",
        calc_optimized_square_solution(parse_string_concatenated(_input))
    );
    // 20048741
}
