use core::panic;
use std::{io::{stdin, Read}, fmt, cmp::Ordering};

use itertools::{Itertools, EitherOrBoth};

#[derive(Clone, Eq, PartialEq)]
enum Packet {
    List(Box<Vec<Packet>>),
    Integer(i32),
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::Integer(i1), Packet::Integer(i2)) => i1.cmp(i2),
            (Packet::List(l1), Packet::List(l2)) => {
                for pair in l1.iter().zip_longest(l2.iter()) {
                    let item_cmp = match pair {
                        EitherOrBoth::Left(_) => Ordering::Greater,  // right ran out first
                        EitherOrBoth::Right(_) => Ordering::Less,  // left ran out first
                        EitherOrBoth::Both(p1, p2) => p1.cmp(p2),
                    };
                    if item_cmp != Ordering::Equal {
                        return item_cmp;
                    }
                }
                return Ordering::Equal;
            },
            (Packet::Integer(i), Packet::List(_)) =>
                Packet::List(vec![Packet::Integer(*i)].into()).cmp(other),
            (Packet::List(_), Packet::Integer(i)) =>
                self.cmp(&Packet::List(vec![Packet::Integer(*i)].into())),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Debug for Packet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Packet::List(l) => write!(f, "{:?}", *l),
            Packet::Integer(i) => write!(f, "{:?}", i),
        }
    }
}

impl From<&str> for Packet {
    fn from(s: &str) -> Self {
        // Parses the first packet it can find in the string s
        assert!(!s.is_empty(), "Packet::from got an empty string");

        let mut stack: Vec<Packet> = Vec::new();
        let mut int_buffer = String::new();
        for c in s.chars() {
            match c {
                '[' => {
                    assert!(int_buffer.is_empty(), "Found '[' but int_buffer is not empty");
                    stack.push(Packet::List(Vec::new().into()));
                },
                '0'..='9' => {
                    int_buffer.push(c);
                },
                ',' => {
                    if !int_buffer.is_empty() {
                        assert!(!stack.is_empty(), "Found ',' but stack is empty");
                        let new_integer = Packet::Integer(int_buffer.parse::<i32>().unwrap());
                        int_buffer.clear();
                        match stack.last_mut().unwrap() {
                            Packet::Integer(_) => panic!("Found ',' but stack.last is Packet::Integer"),
                            Packet::List(list) => list.push(new_integer),
                        };
                    }
                },
                ']' => {
                    assert!(!stack.is_empty(), "Found ']' but stack is empty");
                    if !int_buffer.is_empty() {
                        let new_integer = Packet::Integer(int_buffer.parse::<i32>().unwrap());
                        int_buffer.clear();
                        match stack.last_mut().unwrap() {
                            Packet::Integer(_) => panic!("Found ',' but stack.last is Packet::Integer"),
                            Packet::List(list) => list.push(new_integer),
                        };
                    }
                    let new_list = Packet::List(match stack.pop().unwrap() {
                        Packet::Integer(_) => panic!("Found ']' but stack.last is Packet::Integer"),
                        Packet::List(list) => list,
                    });
                    if stack.is_empty() {
                        return new_list;
                    }
                    assert!(!stack.is_empty(), "Trying to push new list into nothing");
                    match stack.last_mut().unwrap() {
                        Packet::Integer(_) => panic!("Trying to push new list into a Packet::Integer"),
                        Packet::List(list) => list.push(new_list),
                    };
                }
                _ => todo!(),
            }
        }
        assert!(stack.is_empty(), "Parsing complete but stack is not empty");
        if int_buffer.is_empty() {
            println!("stack: {:?}", stack);
        }
        assert!(!int_buffer.is_empty(), "Parsing complete but int_buffer is empty");
        Packet::Integer(int_buffer.parse::<i32>().unwrap())
    }
}

pub fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let num_ordered =
        input
        .split("\n\n")
        .map(|split|
            split.split("\n").take(2).map(|ln| Packet::from(ln)).collect_tuple().unwrap()
        )
        .enumerate()
        .filter_map(|(i, (a, b))| if a < b { Some(i + 1) } else { None })
        .sum::<usize>();

    println!("{}", num_ordered);

    let dividers = vec![Packet::from("[[2]]"), Packet::from("[[6]]")];
    let sorted_packets =
        input
        .lines()
        .filter_map(|ln| if ln.is_empty() { None } else { Some(Packet::from(ln)) })
        .interleave(dividers.clone())
        .sorted()
        .collect_vec();
    let decoder_idxs: (usize, usize) =
        dividers
        .iter()
        .map(|divider| 1 + sorted_packets.iter().position(|packet| packet == divider).unwrap())
        .collect_tuple()
        .unwrap();
    let decoder_key = decoder_idxs.0 * decoder_idxs.1;

    println!("{}", decoder_key);
}
