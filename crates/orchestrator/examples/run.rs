use colored::Colorize;
use llcc_error::LlccB;
use llcc_orchestrator::Src;
use llcc_orchestrator::run;

fn main() -> LlccB<(),> {
	let status = run(Src::Str("5",),)?;
	eprintln!("{}", format!("exit status: {}", status).purple());

	LlccB::X((),)
}
