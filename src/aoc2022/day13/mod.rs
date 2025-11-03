use nom::{
    bytes::complete::tag,
    character::complete::{char, digit1},
    combinator::{map, map_opt},
    multi::separated_list0,
    sequence::{delimited, separated_pair},
    IResult,
};
use std::cmp::Ordering;

#[derive(Debug, Eq)]
enum Packet {
    Number(u8),
    Vector(Vec<Self>),
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Number(a), Self::Number(b)) => a.cmp(b),
            (Self::Vector(_a), Self::Number(b)) => {
                self.cmp(&Packet::Vector(vec![Self::Number(*b)]))
            }
            (Self::Number(a), Self::Vector(_b)) => {
                Packet::Vector(vec![Self::Number(*a)]).cmp(other)
            }
            (Self::Vector(a), Self::Vector(b)) => {
                for (l, r) in a.iter().zip(b) {
                    match l.cmp(r) {
                        Ordering::Less => return Ordering::Less,
                        Ordering::Greater => return Ordering::Greater,
                        Ordering::Equal => {}
                    }
                }
                a.len().cmp(&b.len())
            }
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}

fn part1n2(_input: &str) -> (i32, i32) {
    let t = _input
        .split("\n\n")
        .map(|p| pair(p.as_bytes()).unwrap().1)
        .enumerate()
        .filter(|(_, (a, b))| a.cmp(b) == Ordering::Less);

    let i = t.map(|(i, _)| i + 1);

    (i.sum::<usize>() as i32, 0)
}

#[cfg(test)]
mod tests {
    use super::*;
    static INSTR_SMALL: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn enum_vector_test() {
        let number = Packet::Number(3);
        assert_eq!(Packet::Number(3), number);
        assert_eq!(Packet::Number(3).cmp(&Packet::Number(5)), Ordering::Less);
        assert_eq!(Packet::Number(9).cmp(&Packet::Number(5)), Ordering::Greater);
        assert_eq!(Packet::Number(3).cmp(&Packet::Number(3)), Ordering::Equal);

        let vec1 = Packet::Vector(vec![Packet::Number(1), Packet::Number(6)]);
        let vec2 = Packet::Vector(vec![Packet::Number(3), Packet::Number(4)]);
        let vec3 = Packet::Vector(vec![Packet::Number(3), Packet::Number(1)]);
        let vec4 = Packet::Vector(vec![
            Packet::Vector(vec![Packet::Number(3)]),
            Packet::Number(1),
        ]);

        assert_eq!(vec1.cmp(&vec2), Ordering::Less);
        assert_eq!(vec2.cmp(&vec3), Ordering::Greater);
        assert_eq!(vec3.cmp(&vec2), Ordering::Less);
        assert_eq!(vec3.cmp(&vec4), Ordering::Equal);
    }

    #[test]
    fn examples() {
        assert_eq!((13, 0), part1n2(INSTR_SMALL));
    }
}

pub fn day13() {
    let _input = include_str!("input.txt"); // 5013
    println!("Day13 answers: {:?}", part1n2(_input));
}

fn item(input: &[u8]) -> IResult<&[u8], Packet> {
    nom::branch::alt((map(list, Packet::Vector), map(num, Packet::Number)))(input)
}

fn num(input: &[u8]) -> IResult<&[u8], u8> {
    map_opt(digit1, atoi::atoi)(input)
}

fn list(input: &[u8]) -> IResult<&[u8], Vec<Packet>> {
    delimited(char('['), separated_list0(char(','), item), char(']'))(input)
}

fn pair(input: &[u8]) -> IResult<&[u8], (Packet, Packet)> {
    separated_pair(item, tag("\n"), item)(input)
}
