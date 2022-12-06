use common::{read_lines, Doublet, IterExt, TupleSum};

#[derive(Clone, Copy, Debug)]
struct L(u32);

impl L {
	fn c(self, o: Self) -> u32 {
		(self.partial_cmp(&o).unwrap() as i8 * 3 + 4) as u32 + self.0
	}

	fn p(self, m: Self) -> Self {
		Self((self.0 + m.0 + 2) % 3)
	}
}

impl PartialEq for L {
	fn eq(&self, o: &Self) -> bool {
		self.partial_cmp(o) == Some(std::cmp::Ordering::Equal)
	}
}

impl PartialOrd for L {
	fn partial_cmp(&self, o: &Self) -> Option<std::cmp::Ordering> {
		let (s, o) = (self.0, o.0);
		if s + o == 2 {
			o.partial_cmp(&s)
		} else {
			s.partial_cmp(&o)
		}
	}
}

impl From<char> for L {
	fn from(v: char) -> Self {
		Self((v as u32 - 65) % 23)
	}
}

fn main() {
	let (a, b) = read_lines()
		.filter_map(|l| {
			l.chars()
				.filter(|&c| c != ' ')
				.map(|c| L::from(c))
				.next_tuple::<Doublet<_>>()
		})
		.map(|(a, b)| (b.c(a), a.p(b).c(a)))
		.tuple_sum();
	println!("{a}");
	println!("{b}");
}
