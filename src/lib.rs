#![feature(try_trait_v2)]

use crate::err::B::X;
// use crate::err::B::Y;
use crate::err::LlccB;
use crate::err::ReShape;
use std::fs;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use std::process::ExitStatus;

pub mod err;

/// path to output directory/files
const OUTPUT: &str = "assets/out";
/// path to input directory/files
const INPUT: &str = "assets/in";
/// syscall number of exit
const EXIT: i32 = 93;

pub fn read_src(src: Option<String,>,) -> LlccB<impl Into<String,>,> {
	if let Some(src,) = src {
		return X(src,);
	}

	let mut input = fs::File::open(INPUT,)?;
	let mut buf = String::new();
	input.read_to_string(&mut buf,)?;
	let buf = buf.trim_end().to_string();
	X(buf,)
}

pub fn run(src: impl Into<String,>,) -> LlccB<ExitStatus,> {
	let exec_path = compile(src,)?;
	let status = exec(exec_path,)?;
	X(status,)
}

/// # Return
///
/// returns path to generated executable file
fn compile(src: impl Into<String,>,) -> LlccB<impl Into<PathBuf,>,> {
	let asm_path = emit_asm(src,)?;
	let obj_path = assemble(asm_path,)?;
	let exe_path = link(obj_path,)?;
	X(exe_path,)
}

/// # Return
///
/// returns path to generated assembly file
fn emit_asm(src: impl Into<String,>,) -> LlccB<impl Into<PathBuf,>,> {
	let asm = simplest_asm_str(src,);
	let asm_path = write_asm(asm,)?;
	X(asm_path,)
}

fn simplest_asm_str(src: impl Into<String,>,) -> impl Into<String,> {
	[
		".text",
		".global _start",
		"_start:",
		&format!("\tmov x0, #{}", src.into()),
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
fn write_asm(asm: impl Into<String,>,) -> LlccB<impl Into<PathBuf,>,> {
	let asm_out = format!("{OUTPUT}.s");
	let mut output = fs::OpenOptions::new()
		.truncate(true,)
		.create(true,)
		.write(true,)
		.open(&asm_out,)?;
	output.write_all(asm.into().as_bytes(),)?;
	X(asm_out,)
}

/// # Return
///
/// returns path to generated object file
fn assemble(asm_path: impl Into<PathBuf,>,) -> LlccB<impl Into<PathBuf,>,> {
	let obj_out = format!("{OUTPUT}/out.o");
	Command::new("as",)
		.args(["-o", &obj_out, &stringify_path(asm_path,)?,],)
		.spawn()?;
	X(obj_out,)
}

/// # Return
///
/// returns path to generated executable file
fn link(obj_path: impl Into<PathBuf,>,) -> LlccB<impl Into<PathBuf,>,> {
	let executable = format!("{OUTPUT}/out");
	Command::new("ld",)
		.args(["-o", &executable, &stringify_path(obj_path,)?,],)
		.spawn()?;
	X(executable,)
}

fn exec(exec_path: impl Into<PathBuf,>,) -> LlccB<ExitStatus,> {
	let exec_path = format!("./{}", &stringify_path(exec_path)?);
	eprintln!("{exec_path}");
	let code = Command::new(exec_path,).status()?;
	X(code,)
}

fn stringify_path(path: impl Into<PathBuf,>,) -> LlccB<String,> {
	let p = path.into();
	let p = p.to_str().reshape("failed to get path",)?;
	X(p.to_string(),)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_run() -> LlccB<(),> {
		let exit_code = 0;
		let exit_code_str = format!("{exit_code}");
		let exit_status = run(exit_code_str,)?;
		assert_eq!(Some(exit_code), exit_status.code());
		X((),)
	}
}
