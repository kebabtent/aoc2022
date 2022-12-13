use common::{read_lines, IterExt};

fn main() {
	let (mut x, mut c, mut v) = (1i32, 0i32, 0i32);
	let a: i32 = read_lines()
		.batching(|l| {
			if c > 0 {
				c = 0;
				return Some(x);
			}
			x += v;
			let n = l.next()?;
			v = if n == "noop" {
				0
			} else {
				c = 1;
				n.split(" ").skip(1).next()?.parse().ok()?
			};
			Some(x)
		})
		.enumerate()
		.map(|(i, v)| (i as i32, v))
		.inspect(|&(i, v)| {
			print!("{}", if ((i % 40) - v).abs() <= 1 { "#" } else { " " });
			if (i + 1) % 40 == 0 {
				println!();
			}
		})
		.skip(19)
		.step_by(40)
		.map(|(i, v)| (i + 1) * v)
		.sum();
	println!("{}", a);
}
