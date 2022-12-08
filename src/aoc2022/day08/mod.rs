use itertools::Itertools;

fn parse2d_array(log: &str) -> (i32, i32) {

    let rows = log.lines().into_iter().map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect_vec()).collect::<Vec<_>>().to_vec();
    // TODO take columns from rows without array2D
    let array = array2d::Array2D::from_rows(&rows);

    let side_len = array.row_len();
    let mut visible_count = side_len *2 + side_len *2 - 4;
    let mut high_score = 0;

    for (y, _col) in array.columns_iter().enumerate().take(side_len-1).skip(1)
    {
        for (x, _row) in array.rows_iter().enumerate().take(side_len-1).skip(1)
        {
            let val = array.get(y, x).unwrap();
            //println!("analysis coords x:{} y:{}  val: {}", x,y, val);
            let row = &rows[y];
            let col = &array.as_columns()[x];
            let right_view = &row[x+1..];
            let down_view = &col[y+1..];
            let left_view = &row[..x];
            let up_view = &col[..y];

            let mut left_reverse = left_view.to_owned();
            let mut up_reverse = up_view.to_owned();
            left_reverse.reverse();
            up_reverse.reverse();


            let r = right_view.iter().all(|i| val > i);
            let d = down_view.iter().all(|i| val > i);
            let l = left_reverse.iter().all(|i| val > i);
            let u = up_reverse.iter().all(|i| val > i);

            if r || d || l || u {
                visible_count += 1;
            }

            let rr = right_view.iter().position(|p| p >= val).unwrap_or(right_view.len()-1) + 1;
            let dd = down_view.iter().position(|p| p >= val).unwrap_or(down_view.len()-1) + 1;
            let ll = left_reverse.iter().position(|p| p >= val).unwrap_or(left_reverse.len()-1) + 1;
            let uu = up_reverse.iter().position(|p| p >= val).unwrap_or(up_reverse.len()-1) + 1;

            let score = rr*dd*ll*uu;

            if high_score < score{
                high_score = score;
            }

            /*
            println!("{rr:<3} {r} right {:<?}", right_view);
            println!("{dd:<3} {d} down {:<?}", down_view);
            println!("{ll:<3} {l} left {:<?}", left_reverse);
            println!("{uu:<3} {u} up {:<?}", up_reverse);
 */
        }
    }

    return (visible_count as i32, high_score as i32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part1() {
        let tree_heights = "30373
25512
65332
33549
35390";
        
        assert_eq!((21, 8), parse2d_array(tree_heights));
    }
}

pub fn day08() {
    let _input = include_str!("input.txt");
    println!("Day08 answers: {:?}", parse2d_array(_input)); // ...1779 part2 172224
}
