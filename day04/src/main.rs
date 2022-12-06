use common::{read_lines, IterExt, Quartet, TupleSum};

fn main() {
	let (a, b) = read_lines()
		.filter_map(|l| {
			l.split(|c| c == '-' || c == ',')
				.filter_map(|v| v.parse().ok())
				.next_tuple::<Quartet<u32>>()
		})
		.map(|(a, b, c, d)| {
			(
				(a >= c && b <= d) || (c >= a && d <= b),
				(a >= c && a <= d) || (c >= a && c <= b),
			)
		})
		.tuple_sum();
	println!("{a}");
	println!("{b}");
}
