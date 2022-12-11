#[derive(Debug, Clone)]
struct Monkey {
    id: i32,
    items: Vec<u64>,
    operation: fn(u64) -> u64,
    test: fn(u64) -> bool,
    inspection_count: u64,
}

impl Monkey {
    pub fn new(id: i32, operation: fn(u64) -> u64, test: fn(u64) -> bool) -> Self {
        Self {
            id,
            items: [].to_vec(),
            operation,
            test,
            inspection_count: 0,
        }
    }
}

fn monkey_business(
    monkeys: &mut [&mut Monkey],
    connections: &[(usize, usize)],
    serious_flag: (bool, u64),
) {
    for i in 0..monkeys.len() {
        while !monkeys[i].items.is_empty() {
            let monkey = &mut monkeys[i];
            let item = monkey.items.pop().unwrap();

            let boring_item = match serious_flag.0 {
                false => ((monkey.operation)(item)) / 3,
                true => {
                    let tmp = (monkey.operation)(item);
                    tmp % serious_flag.1
                }
            };

            monkey.inspection_count += 1;
            let test_result = (monkey.test)(boring_item);

            if test_result {
                let true_monkey_id = connections.get(monkey.id as usize).unwrap().0;
                monkeys[true_monkey_id].items.push(boring_item);
            } else {
                let false_monkey_id = connections.get(monkey.id as usize).unwrap().1;
                monkeys[false_monkey_id].items.push(boring_item);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn struct_test() {
        let monkey = Monkey::new(0, |o| o * 6, |d| d % 23 == 0);
        assert!(monkey.items.is_empty());
        assert!((monkey.test)(23));
        assert_eq!(36, (monkey.operation)(6));
    }

    #[test]
    fn example() {
        let mut monkey_0 = Monkey {
            id: 0,
            items: [79, 98].to_vec(),
            operation: |o| o * 19,
            test: |d| d % 23 == 0,
            inspection_count: 0,
        };
        let mut monkey_1 = Monkey::new(1, |o| o + 6, |d| d % 19 == 0);
        monkey_1.items = [54, 65, 75, 74].to_vec();
        let mut monkey_2 = Monkey::new(2, |o| o * o, |d| d % 13 == 0);
        monkey_2.items = [79, 60, 97].to_vec();
        let mut monkey_3 = Monkey::new(3, |o| o + 3, |d| d % 17 == 0);
        monkey_3.items = [74].to_vec();

        let mut serious_monkeys = [
            &mut monkey_0.clone(), 
            &mut monkey_1.clone(), 
            &mut monkey_2.clone(), 
            &mut monkey_3.clone(), 
        ];

        let monkey_tf_connections = [(2, 3), (2, 0), (1, 3), (0, 1)];
        let mut monkeys = [&mut monkey_0, &mut monkey_1, &mut monkey_2, &mut monkey_3];

        for _ in 0..20 {
            monkey_business(&mut monkeys, &monkey_tf_connections, (false, 0));
        }

        monkeys.sort_by_key(|m| m.inspection_count);
        monkeys.reverse();
        assert_eq!(105, monkeys[0].inspection_count);
        assert_eq!(101, monkeys[1].inspection_count);
        assert_eq!(10605, monkeys.iter().take(2).map(|m| m.inspection_count).product::<u64>());

        for _ in 0..10000 {
            monkey_business(&mut serious_monkeys, &monkey_tf_connections, (true, 19 * 13 * 17 * 23));
        }

        serious_monkeys.sort_by_key(|m| m.inspection_count);
        serious_monkeys.reverse();
        assert_eq!(52166, serious_monkeys[0].inspection_count);
        assert_eq!(52013, serious_monkeys[1].inspection_count);
        assert_eq!( 2713310158, serious_monkeys .iter() .take(2) .map(|m| m.inspection_count) .product::<u64>()
        );
    }
}

pub fn day11() {
    let mut monkey_0 = Monkey::new(0, |o| o * 11, |d| d % 5 == 0);
    monkey_0.items = [77, 69, 76, 77, 50, 58].to_vec();
    let mut monkey_1 = Monkey::new(1, |o| o + 8, |d| d % 17 == 0);
    monkey_1.items = [75, 70, 82, 83, 96, 64, 62].to_vec();
    let mut monkey_2 = Monkey::new(2, |o| o * 3, |d| d % 2 == 0);
    monkey_2.items = [53].to_vec();
    let mut monkey_3 = Monkey::new(3, |o| o + 4, |d| d % 7 == 0);
    monkey_3.items = [85, 64, 93, 64, 99].to_vec();
    let mut monkey_4 = Monkey::new(4, |o| o * o, |d| d % 3 == 0);
    monkey_4.items = [61, 92, 71].to_vec();
    let mut monkey_5 = Monkey::new(5, |o| o + 2, |d| d % 11 == 0);
    monkey_5.items = [79, 73, 50, 90].to_vec();
    let mut monkey_6 = Monkey::new(6, |o| o + 3, |d| d % 13 == 0);
    monkey_6.items = [50, 89].to_vec();
    let mut monkey_7 = Monkey::new(7, |o| o + 5, |d| d % 19 == 0);
    monkey_7.items = [83, 56, 64, 58, 93, 91, 56, 65].to_vec();

    let monkey_tf_connections = [
        (1, 5),
        (5, 6),
        (0, 7),
        (7, 2),
        (2, 3),
        (4, 6),
        (4, 3),
        (1, 0),
    ];
    let mut serious_monkeys = [
        &mut monkey_0.clone(),
        &mut monkey_1.clone(),
        &mut monkey_2.clone(),
        &mut monkey_3.clone(),
        &mut monkey_4.clone(),
        &mut monkey_5.clone(),
        &mut monkey_6.clone(),
        &mut monkey_7.clone(),
     ];
    let mut monkeys = [
        &mut monkey_0,
        &mut monkey_1,
        &mut monkey_2,
        &mut monkey_3,
        &mut monkey_4,
        &mut monkey_5,
        &mut monkey_6,
        &mut monkey_7,
    ];

    for _ in 0..20 {
        monkey_business(&mut monkeys, &monkey_tf_connections, (false, 0));
    }

    monkeys.sort_by_key(|m| m.inspection_count);
    monkeys.reverse();

    let monkey_power_level = monkeys
        .iter()
        .take(2)
        .map(|m| m.inspection_count)
        .product::<u64>();

    println!("Day11 part1: {:?}", monkey_power_level); // part1 57838

    for _ in 0..10000 {
        monkey_business(&mut serious_monkeys, &monkey_tf_connections, (true, 5 * 17 * 2 * 7 * 3 * 11 * 13 * 19));
    }
    serious_monkeys.sort_by_key(|m| m.inspection_count);
    serious_monkeys.reverse();

    println!("Day11 part2: {:?}", serious_monkeys.iter().take(2).map(|m| m.inspection_count) .product::<u64>()); // part2 15050382231 
}
