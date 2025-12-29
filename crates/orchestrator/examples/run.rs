use colored::Colorize;
use llcc_error::LlccB;
use llcc_orchestrator::run;

fn main() -> LlccB<(),> {
	let status = run(None::<&str,>,)?;
	eprintln!("{}", format!("exit status: {}", status).purple());

	LlccB::X((),)
}
