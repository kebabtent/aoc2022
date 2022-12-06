use common::{read_lines, IterExt, Triplet, TupleSum};

fn main() {
	let (a, b) = read_lines()
		.map(|l| {
			let mut i = l
				.chars()
				.map(|c| c as u64 - 65)
				.map(|c| (c - 6 * (c / 32) + 26) % 52 + 1);
			let m = i.by_ref().take(l.len() / 2).fold(0u64, |a, x| a | 1 << x);
			i.fold((m, m), |(a, b), x| {
				let a = if m & 1 << x > 0 { x } else { a };
				(a, b | 1 << x)
			})
		})
		.batching(|i| {
			let ((a, x), (b, y), (c, z)) = i.next_tuple::<Triplet<_>>()?;
			Some((a + b + c, (x & y & z).trailing_zeros() as u64))
		})
		.tuple_sum();
	println!("{a}");
	println!("{b}");
}
