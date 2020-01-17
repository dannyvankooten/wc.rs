#![feature(test)]
extern crate test;

use std::env;
use std::fs::File;
use std::io::Read;
use std::env::Args;

#[derive(Debug)]
struct Config {
	lines : bool,
	words : bool,
	chars : bool,
	files: Vec<String>,
}

struct Item {
	filename : String,
	lines: usize,
	words: usize,
	chars: usize,
}

fn main() {
	let c = parse_args(env::args());
	let mut totals = Item{
		filename: String::from("total"),
		lines: 0,
		words: 0,
		chars: 0,
	};
	let mut lines : Vec<Item> = vec![];
	let count_totals = c.files.len() > 0;

	for f in &c.files {
		let item = process_file(f);

		if count_totals {
			totals.lines += item.lines;
			totals.words += item.words;
			totals.chars += item.chars;
		}

		lines.push(item);
	}

	// print results
	for i in lines.iter() {
		print(&c, &i);
	}
	if count_totals {
		print(&c, &totals);
	}
}

fn process_file(f : &str) -> Item {
	let char_whitespace = ' ' as u8;
	let char_newline = '\n' as u8;
	let chunk_size = 0x4000;

	let mut file = File::open(f)
		.expect(&format!("error opening file {}", f));
	let mut item = Item {
		filename: f.to_owned(),
		lines: 0,
		words: 0,
		chars: 0,
	};

	loop {
		let mut chunk = Vec::with_capacity(chunk_size);
		let n = file.by_ref().take(chunk_size as u64)
			.read_to_end(&mut chunk)
			.expect(&format!("error reading file {}", f));

		if n == 0 {
			break;
		}

		for c in chunk {
			item.chars += 1;

			if c == char_whitespace  {
				item.words = item.words + 1
			} else if c == char_newline {
				item.lines += item.lines + 1
			}
		}
	}

	return item;
}

fn print(c : &Config, i : &Item) {
	if c.lines {
		print!("{}", i.lines);
	}
	if c.words {
		print!("\t{}", i.words);
	}
	if c.chars {
		print!("\t{}", i.chars);
	}

	print!("\t{}\n", i.filename);
}

fn parse_args(args : Args) -> Config {
	let mut c = Config{
		lines: false,
		words: false,
		chars: false,
		files: vec![]
	};

	for argument in  args.skip(1) {
		match argument.as_str() {
			"-l" => c.lines = true,
			"-w" => c.words = true,
			"-c" => c.chars = true,
			_ => c.files.push(argument)
		}
	}

	if !c.lines && !c.words && !c.chars {
		c.lines = true;
		c.words = true;
		c.chars = true;
	}

	c
}

#[cfg(test)]
mod tests {
	use super::*;
	use test::Bencher;

	#[test]
	fn test_add() {
		let c = parse_args(env::args());
		assert_eq!(c.lines, true);
		assert_eq!(c.chars, true);
		assert_eq!(c.words, true);
		assert_eq!(0, c.files.len());
	}

	#[bench]
	fn bench_process_file(b: &mut Bencher) {
		b.iter(|| process_file("1.txt"));
	}
}
