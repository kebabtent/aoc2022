use common::{read_chars, Bitmap};

fn main() {
	let g: Vec<_> = read_chars().map(|c| c as usize).collect();
	let (n, _) = g.iter().enumerate().find(|&(_, &c)| c == 10).unwrap();
	let mut b = Bitmap::<256>::new();
	let mut s = vec![1; n * (n + 1)];
	for d in 0..4 {
		for i in 0..n {
			let (mut h, mut o) = (0, [0; 10]);
			for j in 0..n {
				let z = if d / 2 == 0 { j } else { n - 1 - j };
				let (y, z) = if d % 2 == 0 { (i, z) } else { (z, i) };
				let x = y * (n + 1) + z;
				let v = g[x] - 48;
				if i == 0 || j == 0 || v > h {
					b.set(x);
				}
				s[x] *= j - o[v..].iter().max().unwrap();
				h = std::cmp::max(h, v);
				o[v] = j;
			}
		}
	}
	println!("{}", b.cardinality());
	println!("{}", s.iter().max().unwrap());
}
