use itertools::Itertools;

fn scan_beacons(_input: &str, _border: i32) -> i32 {
    let mut sbb: Vec<((i32, i32), (i32, i32))> = Default::default();
    for line in _input.lines() {
        let tmp = line.split_whitespace().collect_vec();
        let x1 = &tmp[2][2..tmp[2].len() - 1].parse::<i32>().unwrap().clone();
        let y1 = &tmp[3][2..tmp[3].len() - 1].parse::<i32>().unwrap().clone();
        let x2 = &tmp[8][2..tmp[8].len() - 1].parse::<i32>().unwrap().clone();
        let y2 = &tmp[9][2..].parse::<i32>().unwrap().clone();

        sbb.push(((*x1, *y1), (*x2, *y2)));
    }

    let mut total = 0;

    //for x in -50..50 {
    for x in -1093990..5000000 {
        for (s, b) in sbb.iter() {
            let xb = manhattan_distance(x, _border, b.0, b.1);
            let xs = manhattan_distance(x, _border, s.0, s.1);
            let sb = manhattan_distance(b.0, b.1, s.0, s.1);

            let xbs = xb < xs && sb >= xs;
            let xsb = xs < xb && sb >= xs;
            let bxs = xs < sb && xb <= sb;

            //println!("{x} {s:2?} {b:2?}  {xb:2} {xs:2} {SB:2} {xbs} {xsb} {bxs}");

            if xbs || xsb || bxs {
                total += 1;
                break;
            }

            /*
            if xs >= xb && SB > xs {
                total += 1;
                break;
            //} else if xs < xb && SB > xs {
            } else if xs < xb {
                total += 1;
                break;
            } */
        }
    }

    total
}

// from minutiae
pub fn manhattan_distance(x1: i32, y1: i32, x2: i32, y2: i32) -> i32 {
    let x_diff = if x1 < x2 { x2 - x1 } else { x1 - x2 };
    let y_diff = if y1 < y2 { y2 - y1 } else { y1 - y2 };
    x_diff + y_diff
}

#[cfg(test)]
mod tests {
    use super::*;
    static INSTR_SMALL: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    #[test]
    fn examples() {
        assert_eq!(26, scan_beacons(INSTR_SMALL, 10));
    }
}

pub fn day15() {
    let _input = include_str!("input.txt");
    println!("Day15 part1: {:?}", scan_beacons(_input, 2000000)); // 4796338 // too high :) i have smth wrong
}
