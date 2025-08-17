use itertools::Itertools;


fn rules(color: &str) -> u32 {
    match color {
        "blue" => 14,
        "green" => 13,
        "red" => 12,
        _ => 0
    }
}

#[derive(Debug)]
struct MaxColor {
    blue: u32,
    green: u32,
    red: u32,
}

impl MaxColor {
    fn mul(&self) -> u32 {
        self.blue*self.green*self.red
    }
}

fn update_color_max_value(max_color: &mut MaxColor, new_max: (u32, &str)) {
    match new_max {
        (value, "blue") if value > max_color.blue => max_color.blue = value,
        (value, "green") if value > max_color.green => max_color.green = value,
        (value, "red") if value > max_color.red => max_color.red = value,
        _ => (),
    }
}

fn is_possible(game: &str, is_part2: bool) -> u32 {
    let mut max_color = MaxColor{ blue: 0, green: 0, red: 0};
    let mut enough_balls = true;

    let (id, games) =  game.split_once(':').unwrap();
    let rounds: Vec<_> = games.split(';').to_owned().collect();

    for round in rounds{
        let balls : Vec<_> = round.split(',').collect();
        for ball in balls { // N color
            let (number, color) = ball.trim()
            .split_once(' ')
            .map(|(a, b)| (a.parse::<u32>().unwrap(), b))
            .unwrap();

            update_color_max_value(&mut max_color, (number, color));

            if number > rules(color) {
                enough_balls = false;
            }
        }
    }

    if is_part2 {
        max_color.mul()
    } else {
        match enough_balls {
            false => 0,
            true => id.split_whitespace().last().to_owned().unwrap().parse::<u32>().unwrap()
        }
    }

}

fn is_possible_concise(game: &str) -> u32 {
    let (id, games) = game.split_once(':').unwrap();

    id.split_whitespace().last().unwrap().parse::<u32>().unwrap()
        * !games.split(';')
            .flat_map(|round| round.split(','))
            .map(|ball| ball.split_whitespace().collect_tuple().unwrap())
            .any(|(number, color)| number.parse::<u32>().unwrap() > rules(color)) as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    static INSTR_SMALL: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn examples() {
        let games: Vec<&str> = INSTR_SMALL.lines().into_iter().collect();

        assert_eq!(1, is_possible(games[0], false));
        assert_eq!(2, is_possible(games[1], false));
        assert_eq!(0, is_possible(games[2], false));
        assert_eq!(0, is_possible(games[3], false));
        assert_eq!(5, is_possible(games[4], false));

        assert_eq!(1, is_possible_concise(games[0]));
        assert_eq!(2, is_possible_concise(games[1]));
        assert_eq!(0, is_possible_concise(games[2]));
        assert_eq!(0, is_possible_concise(games[3]));
        assert_eq!(5, is_possible_concise(games[4]));

        // part2 
        assert_eq!(48, is_possible(games[0], true));
        assert_eq!(12, is_possible(games[1], true));
        assert_eq!(1560, is_possible(games[2], true));
        assert_eq!(630, is_possible(games[3], true));
        assert_eq!(36, is_possible(games[4], true));
    }
}


pub fn day02() {
    let games = include_str!("input.txt");
    
    let result : u32 = games.lines().map(is_possible_concise).sum();
    let result2 : u32 = games.lines().map(|line| is_possible(line, true)).sum();

    println!("Day02 part1: {}", result); // 2369
    println!("Day02 part2: {}", result2); // 66363
}