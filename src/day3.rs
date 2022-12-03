use std::collections::HashSet;
use std::io::{self, BufRead};

fn prio(c: &char) -> u32 {
	if 'a' <= *c && *c <= 'z' {
		return *c as u32 - 'a' as u32 + 1;
	} else if 'A' <= *c && *c <= 'Z' {
		return *c as u32 - 'A' as u32 + 27;
	}
	return ' ' as u32;
}

pub fn main1() {
	let result: u32 =
		io::stdin()
		.lock()
		.lines()
		.map(|ln| ln.unwrap())
		.map(
			|ln|
			HashSet::<char>::from_iter(ln[..ln.len()/2].chars())
				.intersection(
					&HashSet::<char>::from_iter(ln[ln.len()/2..].chars())
				)
				.map(|c| prio(c))
				.sum::<u32>()
		).sum();
	println!("{} ", result);
}

pub fn main2() {
	let lines: Vec<String> =
		io::stdin()
		.lock()
		.lines()
		.map(|ln| ln.unwrap())
		.collect();
	let result: u32 = lines
		.chunks(3)
		.map(
			|group|
			group
				.iter()
				.map(|elf| HashSet::<char>::from_iter(elf.chars()))
				.collect::<Vec<_>>()
		)
		.map(|group| {
			HashSet::<char>::from_iter(
				group[0].intersection(&group[1]).map(|c| *c)
			)
			.intersection(&group[2])
			.map(prio)
			.sum::<u32>()
		}).sum();
	println!("{} ", result);
}
