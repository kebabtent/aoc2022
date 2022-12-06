use common::{read_all_lines, Buffer, IterExt};

fn main() {
	let b = read_all_lines()
		.peekable()
		.batching(|t| {
			t.peek()?;
			Some(
				t.take_while(|l| !l.is_empty())
					.filter_map(|l| l.parse::<u32>().ok())
					.sum::<u32>(),
			)
		})
		.fold(Buffer::new(3), |mut b, v| {
			b.insert_sorted(v);
			b
		});

	println!("{}", b[2]);
	println!("{}", b.iter().sum::<u32>())
}
