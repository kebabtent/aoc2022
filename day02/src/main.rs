use common::{read_lines, Doublet, IterExt, TupleSum};

fn c(l: u32, r: u32) -> u32 {
	let mut c = l.partial_cmp(&r).unwrap();
	if l + r == 2 {
		c = c.reverse();
	}
	(c as i8 * 3 + 4) as u32 + l
}

fn main() {
	let (a, b) = read_lines()
		.filter_map(|l| {
			l.chars()
				.filter(|&c| c != ' ')
				.map(|c| (c as u32 - 65) % 23)
				.next_tuple::<Doublet<_>>()
		})
		.map(|(a, b)| (c(b, a), c((a + b + 2) % 3, a)))
		.tuple_sum();
	println!("{a}");
	println!("{b}");
}
