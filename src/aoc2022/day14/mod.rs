use itertools::Itertools;

fn drop_sand(_input: &str, _endless: bool) -> i32 {
        let mut map = vec![vec![0; 1000]; 1000];
        for line in _input.lines() {
            let walls = line.split(" -> ")
            .collect::<Vec<&str>>();
            let coords = walls.iter()
            .map(|xy| {
                let t = xy.split_once(',')
                .unwrap();
                (t.0.parse::<i32>().unwrap(), t.1.parse::<i32>().unwrap())
            }).collect::<Vec<(i32, i32)>>();


            for (p1, p2) in coords.iter().tuple_windows() {
                // cartesian product would be nice here

                let mut _x = [p1.0, p2.0];
                _x.sort();
                let mut _y = [p1.1, p2.1];
                _y.sort();

                for x in _x[0].._x[1]+1{
                    for y in _y[0].._y[1]+1{
                        map[y as usize][x as usize] = 1;
                    }
                }
            }
        }

        if !_endless
        {
            let y_max = map.iter().position_max_by_key(|y| y.contains(&1)).unwrap();
            for x in 0..999 {
                map[y_max+2][x] = 1;
            }
        }

        let source = (500, 0);

        let mut added = true;
        let mut watchdog = 0;
        let _watchdog_lmt = 300;
        while added {
            let mut pt = source;

            // part2 check
            if map[source.1][source.0] == 2
            {
                break;
            }

            let mut moved = true;
            while moved {

                if map[pt.1+1][pt.0] == 0  {
                    watchdog += 1;
                }

                if watchdog > _watchdog_lmt {
                    added = false;
                    break;
                }

                if map[pt.1+1][pt.0] == 0 {
                   pt.1 +=1;
                   continue;
                } else if map[pt.1+1][pt.0-1] == 0 {
                   pt.1 +=1;
                   pt.0 -=1;
                   continue;
                } else if map[pt.1+1][pt.0+1] == 0 {
                   pt.1 +=1;
                   pt.0 +=1;
                   continue;
                } else {
                    map[pt.1][pt.0] = 2;
                    moved = false;
                    watchdog = 0;
                }

            }
        }
        /* 
        for y in 0..180 {
            let mut line = String::from("");
            for x in 480..571 {
                line += match map[y as usize][x as usize] {
                    0 => ".",
                    1 => "#",
                    2 => "o",
                    _ => unreachable!()
                };
            }
            println!("{line}");
        }
        */
        let cnt = map.iter().flatten().counts()[&2];

    cnt as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    static INSTR_SMALL: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    static INSTR_BIG: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
0,11 -> 999,11";

    #[test]
    fn examples() {
        assert_eq!(24, drop_sand(INSTR_SMALL, true));
        assert_eq!(93, drop_sand(INSTR_BIG, false));
    }
}

pub fn day14() {
    let _input = include_str!("input.txt"); 
    println!("Day14 part1: {:?}", drop_sand(_input, true)); // 1001
    println!("Day14 part2: {:?}", drop_sand(_input, false)); // 27976
}
