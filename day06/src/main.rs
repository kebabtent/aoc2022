use common::{read_chars, Array, IterExt};

fn main() {
	let g = (1u64 << 4) - 1;
	let h = (1u64 << 14) - 1;
	let (a, b) = read_chars()
		.map(|c| c as u64 - 65)
		.tuple_windows::<Array<14, _>>()
		.map(|w| {
			w.into_iter()
				.scan(0u64, |c, x| {
					let o = *c;
					*c |= 1 << x;
					Some(*c != o)
				})
				.enumerate()
				.filter(|&(_, f)| f)
				.fold(0u64, |a, (n, _)| a | 1 << n)
		})
		.enumerate()
		.fold((4, 14), |(mut a, mut b), (n, f)| {
			a += (f & g == g && a == 4) as usize * n;
			b += (f & h == h && b == 14) as usize * n;
			(a, b)
		});
	println!("{a}");
	println!("{b}");
}
