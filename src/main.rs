extern crate fakelish;
use clap::Parser;
use regex::Regex;

#[derive(Parser, Debug)]
#[command(version, about = "English-like word generator", long_about = None)]
struct Args {
	#[arg(short, long, default_value = "6:9", help = "length of word expressed as range")]
	len: String,
	#[arg(short, long, default_value = "4", help = "maximum statistical sequence")]
	seq: u8,
	#[arg(short, long, default_value = "1", help = "number of words")]
	num: u16
}
struct Range {
	min: u8,
	max: u8
}

impl Range {
	pub fn from(arg: &str) -> Self {
		let mut min: u8 = 6;
		let mut max: u8 = 9;
		if Regex::new(r"^\d+:\d+$").unwrap().is_match(arg) {
			let len: Vec<u8> = arg.split(':')
				.map(|m| m.parse().unwrap()).collect();
			min = len[0];
			max = len[1];
		} else {
			match arg.parse::<u8>() {
				Ok(m) => {
					min = m;
					max = m;
				},
				Err(_) => {}
			}
		}
		Range { min, max }
	}
}

fn main() {
	let args = Args::parse();
	let len = Range::from(&args.len);
	let seq: u8 = args.seq;
	let num: u16 = args.num;
	let mut generate = fakelish::WordGenerator::new(len.min, len.max, seq);
	let words = generate.phrase(num);
	println!("{}", words.join("\n"));
}
