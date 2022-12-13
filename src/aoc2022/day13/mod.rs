use std::cmp::Ordering;
#[derive(Debug, Eq)]
enum Packet {
    Number(i32),
    Vector(Vec<Self>),
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Number(a), Self::Number(b)) => a.cmp(b),
            (Self::Vector(_a), Self::Number(b)) => self.cmp(&Packet::Vector(vec![Self::Number(*b)])), 
            (Self::Number(a), Self::Vector(_b)) => Packet::Vector(vec![Self::Number(*a)]).cmp(other),
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
    for line in _input.split("\n\n"){
        let (l, r) = line.split_once('\n').unwrap();
        println!("l: {l:?} r: {r:?} ");
        //for c in l:
        
    }

    (0, 0)
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

    fn parse_array(array :&str) -> Vec::<Packet> {
        let mut vector = Vec::<Packet>::new();
        let mut context = String::from("");
        let chars = array.chars();
        let mut char_cnt = 0;

        for c in  chars {
            match c {
                '[' => {
                    let tmp = array.split_at(char_cnt+1).1;

                    let t = Packet::Vector(parse_array(tmp));
                    //let mut tmp = vector.pop().unwrap();
                    //vector.push(_current);
                    // create new use new
                    //vector.push(Packet::Vector(Vec::<Packet>::new()));
                },
                ',' => {
                    // parse current
                    let val = context.parse::<i32>().unwrap();
                    vector.push(Packet::Number(val));
                    context = String::from("");
                },
                ']' => {
                    if !context.is_empty() {
                        let val = context.parse::<i32>().unwrap();
                        context = String::from("");
                        vector.push(Packet::Number(val));
                    }
                    break;
                },
                _ => {
                    //let x = i.pee
                    context.push(c);
                }
            }
            char_cnt+=1;
        }





        vector
    }

    #[test]
    //fn parse_packet(str) -> Packet{
    fn parse_packet(){
        let packet_easy  = "[1,1,5,10,1]";
        let packet_med = "[[1],[2,3,4]]";
        let t = parse_array(packet_easy);

        assert_eq!(t, vec![Packet::Number(1), Packet::Number(1), Packet::Number(5), Packet::Number(10), Packet::Number(1)]);
        let t2 = parse_array(packet_med);
        assert_eq!(t2.len(), 2);
    }

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
        let vec4 = Packet::Vector(vec![Packet::Vector(vec![Packet::Number(3)]), Packet::Number(1)]);

        assert_eq!(vec1.cmp(&vec2), Ordering::Less);
        assert_eq!(vec2.cmp(&vec3), Ordering::Greater);
        assert_eq!(vec3.cmp(&vec2), Ordering::Less);
        assert_eq!(vec3.cmp(&vec4), Ordering::Equal);
    }

    #[test]
    fn examples() {
        assert_eq!((0, 0), part1n2(INSTR_SMALL));
    }
}

pub fn day13() {
    let _input = include_str!("input.txt"); //437 part1 // part2 430
    //println!("Day13 answers: {:?}", part1n2(_input));
}
