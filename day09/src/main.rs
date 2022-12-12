use common::{read_lines, IterExt};
use std::collections::HashSet;

fn main() {
	let mut x = [0i32; 20];
	let mut a = HashSet::with_capacity(1024);
	a.insert((0, 0));
	let mut b = a.clone();
	for (d, l) in read_lines().filter_map(|l| {
		let (d, l): (&str, &str) = l.split(" ").next_tuple()?;
		Some((
			(d.chars().next()? as u32).saturating_sub(75) as i32,
			l.parse::<usize>().ok()?,
		))
	}) {
		for _ in 0..l {
			let k = (-1i32).pow((7u32.saturating_sub(d as u32)) / 6);
			x[0] += k * (d % 2);
			x[1] += k - k * (d % 2);
			for q in (2..20).step_by(2) {
				if (x[q] - x[q - 2]).abs() <= 1 && (x[q - 1] - x[q + 1]).abs() <= 1 {
					continue;
				}
				for r in 0..2 {
					x[q + r] += (x[q + r - 2] - x[q + r]).signum();
				}
			}
			a.insert((x[2], x[3]));
			b.insert((x[18], x[19]));
		}
	}
	println!("{}", a.len());
	println!("{}", b.len());
}
