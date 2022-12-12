use petgraph::algo::dijkstra;
use petgraph::prelude::*;
use petgraph::Graph;

fn part1n2(_input: &str) -> (i32, i32) {
    let mut graph: Graph<(), (), Directed> = Graph::new();

    let table: Vec<&[u8]> = _input
        .split(|b| b == '\n')
        .map(|s| s.as_bytes())
        .filter(|l| !l.is_empty())
        .collect();

    let word_length = table[0].len();
    let mut start_coord = (0, 0);
    let mut end_coord = (0, 0);
    let mut part2_starters = Vec::new();

    let mut index_table: Vec<Vec<NodeIndex>> = Default::default();

    // create vertices
    for y in 0..table.len() {
        let mut index_row: Vec<NodeIndex> = Default::default();
        for x in 0..word_length {
            let c = table[y][x];

            match c {
                b'`' => start_coord = (x, y),
                b'{' => end_coord = (x, y),
                b'a' => part2_starters.push((x, y)),
                _ => {}
            }
            index_row.push(graph.add_node(()));
        }
        index_table.push(index_row);
    }

    //connect vertices
    for y in 0..table.len() {
        for x in 0..word_length {
            let c = table[y][x];
            if y > 0 {
                let u = table[y - 1][x];
                if u <= c + 1 {
                    graph.update_edge(index_table[y][x], index_table[y - 1][x], ());
                }
            }

            if y < table.len() - 1 {
                let d = table[y + 1][x];
                if d <= c + 1 {
                    graph.update_edge(index_table[y][x], index_table[y + 1][x], ());
                }
            }

            if x < word_length - 1 {
                let r = table[y][x + 1];
                if r <= c + 1 {
                    graph.update_edge(index_table[y][x], index_table[y][x + 1], ());
                }
            }

            if x > 0 {
                let l = table[y][x - 1];
                if l <= c + 1 {
                    graph.update_edge(index_table[y][x], index_table[y][x - 1], ());
                }
            }
        }
    }

    let start = index_table[start_coord.1][start_coord.0];
    let end = index_table[end_coord.1][end_coord.0];
    let res = dijkstra(&graph, start, Some(end), |_| 1);

    // part 2
    let tmp = part2_starters
        .iter()
        .map(|s| {
            let start = index_table[s.1][s.0];
            dijkstra(&graph, start, Some(end), |_| 1)
        })
        .filter_map(|i| match i.contains_key(&end) {
            true => Some(i[&end]),
            false => None,
        })
        .min()
        .unwrap();

    (res[&end], tmp)
}

#[cfg(test)]
mod tests {
    use super::*;
    static INSTR_SMALL: &str = "`abqponm
abcryxxl
accsz{xk
acctuvwj
abdefghi";

    #[test]
    fn char_compare_test() {
        let a = b'a';
        let b = b'b';
        assert!(a < b);
        assert_eq!(1, b - a);
        assert_eq!(0, 0); //31 steps
    }

    #[test]
    fn examples() {
        assert_eq!((31, 29), part1n2(INSTR_SMALL));
    }
}

pub fn day12() {
    let _input = include_str!("input.txt"); //437 part1 // part2 430
    println!("Day12 answers: {:?}", part1n2(_input));
}
