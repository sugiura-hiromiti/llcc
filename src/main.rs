use colored::Colorize;
use llcc::err::LlccB;
use llcc::read_src;
use llcc::run;

fn main() -> LlccB<(),> {
	let src = read_src(None,)?;
	let status = run(src,)?;
	eprintln!("{}", format!("exit status: {}", status).purple());

	LlccB::X((),)
}
