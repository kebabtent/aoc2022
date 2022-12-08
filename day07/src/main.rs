use common::{read_lines, Either, Either::*};

fn t(m: &Vec<Either<Vec<usize>, usize>>, g: usize, i: usize) -> (usize, usize, usize) {
	let (s, mut c, mut f) = m[i]
		.left()
		.iter()
		.skip(1)
		.fold((0, 0, usize::MAX), |(s, c, f), &j| {
			let (x, y, z) = match &m[j] {
				L(_) => t(m, g, j),
				R(v) => (*v, 0, usize::MAX),
			};
			(s + x, c + y, std::cmp::min(f, z))
		});
	if s <= 100000 {
		c += s;
	}
	if s >= g && s < f {
		f = s;
	}
	(s, c, f)
}

fn main() {
	let mut m = vec![L(vec![0])];
	let (mut n, mut i, mut p, mut g) = (0, 0, 0, 0);
	for l in read_lines()
		.skip(1)
		.filter(|l| l != "$ ls" && !l.starts_with("dir"))
	{
		if l == "$ cd .." {
			n = p;
			p = m[n].left()[0];
		} else {
			i += 1;
			m[n].left_mut().push(i);
			if l.starts_with("$ cd ") {
				m.push(L(vec![n]));
				p = n;
				n = i;
			} else {
				let s = l.split(" ").next().and_then(|l| l.parse().ok()).unwrap();
				m.push(R(s));
				g += s;
			}
		}
	}
	let (_, a, b) = t(&m, g - 40000000, 0);
	println!("{a}");
	println!("{b}");
}
