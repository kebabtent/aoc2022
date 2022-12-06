use common::{read_lines, IterExt, Quartet, TupleSum};
use std::cmp::{max, min};

fn main() {
	let (a, b) = read_lines()
		.filter_map(|l| {
			l.split(|c| c == '-' || c == ',')
				.filter_map(|v| v.parse().ok())
				.next_tuple::<Quartet<i32>>()
		})
		.map(|(a, b, c, d)| {
			let o = min(b, d) - max(a, c);
			(o == min(b - a, d - c), o >= 0)
		})
		.tuple_sum();
	println!("{a}");
	println!("{b}");
}
