use anyhow::Context;
use anyhow::Result as Rslt;
use colored::Colorize;
use std::fs;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use std::process::ExitStatus;

/// path to output directory/files
const OUTPUT: &str = "assets/out";
/// path to input directory/files
const INPUT: &str = "assets/in";
/// syscall number of exit
const EXIT: i32 = 93;

fn main() -> Rslt<(),> {
	let src = read_src(None,)?;
	let status = run(src,)?;
	eprintln!("{}", format!("exit status: {}", status).purple());

	Ok((),)
}

fn read_src(src: Option<String,>,) -> Rslt<String,> {
	if let Some(src,) = src {
		return Ok(src,);
	}

	let mut input = fs::File::open(INPUT,)?;
	let mut buf = String::new();
	input.read_to_string(&mut buf,)?;
	let buf = buf.trim_end().to_string();
	Ok(buf,)
}

fn run(src: String,) -> Rslt<ExitStatus,> {
	let exec_path = compile(src,)?;
	let status = exec(exec_path,)?;
	Ok(status,)
}

/// # Return
///
/// returns path to generated executable file
fn compile(src: String,) -> Rslt<String,> {
	let asm_path = emit_asm(src,)?;
	let obj_path = assemble(asm_path,)?;
	let exe_path = link(obj_path,)?;
	Ok(exe_path,)
}

/// # Return
///
/// returns path to generated assembly file
fn emit_asm(src: String,) -> Rslt<String,> {
	let asm = simplest_asm_str(src,);
	let asm_path = write_asm(asm,)?;
	Ok(asm_path,)
}

fn simplest_asm_str(src: impl AsRef<str,>,) -> String {
	[
		".text",
		".global _start",
		"_start:",
		&format!("\tmov x0, #{}", src.as_ref()),
		&format!("\tmov x8, #{EXIT}"),
		"\tsvc #0",
	]
	.join("\n",)
	.trim()
	.to_string()
}

/// # Return
///
/// returns path to generated assembly file
fn write_asm(asm: String,) -> Rslt<String,> {
	let asm_out = format!("{OUTPUT}.s");
	let mut output = fs::OpenOptions::new()
		.truncate(true,)
		.create(true,)
		.write(true,)
		.open(&asm_out,)?;
	output.write_all(asm.as_bytes(),)?;
	Ok(asm_out,)
}

/// # Return
///
/// returns path to generated object file
fn assemble(asm_path: impl AsRef<Path,>,) -> Rslt<String,> {
	let obj_out = format!("{OUTPUT}/out.o");
	Command::new("as",)
		.args(["-o", &obj_out, &stringify(asm_path,)?,],)
		.spawn()?;
	Ok(obj_out,)
}

/// # Return
///
/// returns path to generated executable file
fn link(obj_path: impl AsRef<Path,>,) -> Rslt<String,> {
	let executable = format!("{OUTPUT}/out");
	Command::new("ld",)
		.args(["-o", &executable, &stringify(obj_path,)?,],)
		.spawn()?;
	Ok(executable,)
}

fn exec(exec_path: impl AsRef<Path,>,) -> Rslt<ExitStatus,> {
	let exec_path = format!("./{}", &stringify(exec_path)?);
	eprintln!("{exec_path}");
	let code = Command::new(exec_path,).status()?;
	Ok(code,)
}

fn stringify(path: impl AsRef<Path,>,) -> Rslt<String,> {
	let p = path.as_ref().to_str().context("failed to get path",)?;
	Ok(p.to_string(),)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_run() {}
}
