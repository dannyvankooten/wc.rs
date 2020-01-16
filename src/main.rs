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
		let mut file = File::open(f)
			.expect(&format!("error opening file {}", f));
		let mut content = String::new();

		// TODO: Read buffered instead of all at once
		file.read_to_string(&mut content)
			.expect(&format!("error reading file {}", f));

		// TODO: Iterate over content instead of splitting
		let mut item = Item {
			filename: f.to_owned(),
			lines: 0,
			words: 0,
			chars: 0,
		};

		for c in content.chars() {
			item.chars += 1;

			if c == ' ' {
				item.words = item.words + 1
			} else if c == '\n' {
				item.lines += item.lines + 1
			}
		}

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
mod test {
	use super::*;

	#[test]
	fn test_add() {
		let c = parse_args(env::args());
		assert_eq!(c.lines, true);
		assert_eq!(c.chars, true);
		assert_eq!(c.words, true);
		assert_eq!(0, c.files.len());
	}
}
