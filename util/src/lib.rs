#![feature(duration_zero, test)]

use std::{
	hint::black_box,
	io::Write,
	time::{Duration, Instant},
};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
pub use util_proc_macro::*;

use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

fn format_duration(d: Duration) -> String {
	if d < Duration::from_millis(2) {
		format!("{}μs", d.as_micros())
	} else if d < Duration::from_secs(2) {
		format!("{:.2}ms", d.as_micros() as f64 / 1000.0)
	} else {
		format!("{:.2}s", d.as_secs_f64())
	}
}

pub fn run_benchmark(mut main: impl Send + FnMut() -> String, file: &str) {
	let mut stdout = StandardStream::stdout(ColorChoice::Auto);

	stdout.set_color(ColorSpec::new().set_fg(None)).unwrap();
	write!(&mut stdout, "{: >2} ", &file[8..(file.len() - 3)]).unwrap();

	stdout.flush().unwrap();

	let mut data = Vec::new();
	let mut total = Duration::ZERO;

	let start = Instant::now();

	while total < Duration::from_secs(10) && data.len() < 5000 {
		let start = Instant::now();
		black_box(main());
		let elapsed = start.elapsed();
		total += elapsed;
		data.push(elapsed);
	}

	let elapsed = start.elapsed();

	stdout
		.set_color(ColorSpec::new().set_fg(Some(Color::Green)))
		.unwrap();

	write!(
		&mut stdout,
		"{} ",
		format_duration(data.iter().min().copied().unwrap())
	)
	.unwrap();

	stdout
		.set_color(ColorSpec::new().set_fg(Some(Color::Blue)))
		.unwrap();

	write!(
		&mut stdout,
		"{} ",
		format_duration(total / data.len() as u32)
	)
	.unwrap();

	stdout
		.set_color(ColorSpec::new().set_fg(Some(Color::Ansi256(102))))
		.unwrap();

	write!(
		&mut stdout,
		"[×{} = {}] ± {}",
		data.len(),
		format_duration(total),
		format_duration(elapsed - total),
	)
	.unwrap();

	stdout.set_color(ColorSpec::new().set_fg(None)).unwrap();

	write!(&mut stdout, " → {}", main()).unwrap();

	writeln!(&mut stdout).unwrap();
}
