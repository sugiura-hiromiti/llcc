use colored::Colorize;
use llcc::err::LlccB;
use llcc::front::run;

fn main() -> LlccB<(),> {
	let status = run(None::<&str,>,)?;
	eprintln!("{}", format!("exit status: {}", status).purple());

	LlccB::X((),)
}
