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
				let x = match d {
					0 => i * (n + 1) + j,
					1 => j * (n + 1) + i,
					2 => i * (n + 1) + n - 1 - j,
					_ => (n - 1 - j) * (n + 1) + i,
				};
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
