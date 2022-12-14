use common::{read_lines, IterExt, Triplet};
use std::collections::VecDeque;

fn main() {
	let mut i = read_lines();
	let s = i.by_ref().take_while(|l| l.starts_with('[')).fold(
		vec![VecDeque::with_capacity(16); 10],
		|mut a, l| {
			for (n, c) in l
				.chars()
				.skip(1)
				.step_by(4)
				.enumerate()
				.filter(|&(_, c)| c != ' ')
			{
				a[n].push_front(c);
			}
			a
		},
	);

	let (a, b) = i.fold((s.clone(), s), |(mut a, mut b), l| {
		let (n, f, t) = l
			.split(' ')
			.skip(1)
			.step_by(2)
			.filter_map(|v| v.parse().ok())
			.next_tuple::<Triplet<usize>>()
			.unwrap();
		let c = b[f - 1].len() - n;
		b[f - 1].rotate_left(c);
		for _ in 0..n {
			let c = a[f - 1].pop_back().unwrap();
			a[t - 1].push_back(c);
			let c = b[f - 1].pop_front().unwrap();
			b[t - 1].push_back(c);
		}
		(a, b)
	});

	println!("{}", a.iter().filter_map(|c| c.back()).collect::<String>());
	println!("{}", b.iter().filter_map(|c| c.back()).collect::<String>());
}
