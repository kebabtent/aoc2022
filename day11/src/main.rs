use common::{read_lines, Either::*};
use std::collections::VecDeque;

fn e() -> Option<()> {
	let mut l = read_lines();
	let (mut h, mut o, mut d, mut t, mut f) = (vec![], vec![], vec![], vec![], vec![]);
	loop {
		if l.next().is_none() {
			break;
		}
		h.push(
			l.next()?
				.split(": ")
				.last()?
				.split(", ")
				.filter_map(|v| v.parse().ok())
				.collect::<VecDeque<usize>>(),
		);

		let s = l.next()?;
		let mut s = s.split(" ").skip(6);
		o.push(match (s.next()?, s.next()?.parse::<usize>()) {
			("+", Ok(v)) => Some(L(v)),
			("*", Ok(v)) => Some(R(v)),
			_ => None,
		});
		let v = [&mut d, &mut t, &mut f];
		let mut l = l
			.by_ref()
			.filter_map(|l| l.split(" ").last()?.parse::<usize>().ok());
		for j in 0..3 {
			v[j].push(l.next()?);
		}
	}
	let n = h.len();

	let mut s = Vec::with_capacity(n);
	for m in 0..n {
		let mut u = VecDeque::with_capacity(h[m].len());
		for i in 0..h[m].len() {
			let mut v = Vec::with_capacity(n);
			for j in 0..n {
				v.push(h[m][i] % d[j]);
			}
			u.push_back(v);
		}
		s.push(u);
	}

	let mut c = vec![0; n];
	for _ in 0..20 {
		for m in 0..n {
			c[m] += h[m].len();
			while let Some(mut w) = h[m].pop_front() {
				w = match o[m] {
					Some(L(v)) => w + v,
					Some(R(v)) => w * v,
					None => w * w,
				};
				w /= 3;
				let j = if w % d[m] == 0 { t[m] } else { f[m] };
				h[j].push_back(w);
			}
		}
	}
	c.sort_by(|a, b| b.cmp(a));
	let a = c[0] * c[1];

	let mut c = vec![0; n];
	for _ in 0..10000 {
		for m in 0..n {
			c[m] += s[m].len();
			while let Some(mut w) = s[m].pop_front() {
				for i in 0..n {
					w[i] = match o[m] {
						Some(L(v)) => w[i] + v,
						Some(R(v)) => w[i] * v,
						None => w[i] * w[i],
					} % d[i];
				}
				let j = if w[m] == 0 { t[m] } else { f[m] };
				s[j].push_back(w);
			}
		}
	}
	c.sort_by(|a, b| b.cmp(a));
	let b = c[0] * c[1];

	println!("{a}");
	println!("{b}");

	Some(())
}

fn main() {
	e();
}
