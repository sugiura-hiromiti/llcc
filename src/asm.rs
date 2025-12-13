use crate::err::B::X;
use crate::err::LlccB;
use crate::file_io::Dest;
use crate::file_io::DestKind;
use std::ffi::OsStr;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use std::process::ExitStatus;

/// syscall number of exit
const EXIT: i32 = 93;

pub fn simplest_asm_str(src: impl Into<String,>,) -> impl Into<String,> {
	[
		".text",
		".global _start",
		"_start:",
		&format!("\tmov x0, #{}", src.into()),
		&format!("\tmov x8, #{EXIT}"),
		"\tsvc #0\n",
	]
	.join("\n",)
}

/// # Return
///
/// returns path to generated assembly file
pub fn write_asm(
	asm_src: impl Into<String,>,
	asm_path: impl Into<PathBuf,>,
) -> LlccB<impl Into<PathBuf,>,> {
	let asm_path = asm_path.into();
	let mut output = fs::OpenOptions::new()
		.truncate(true,)
		.create(true,)
		.write(true,)
		.open(&asm_path,)?;
	output.write_all(asm_src.into().as_bytes(),)?;
	X(asm_path,)
}

pub fn run_cmd<I, S,>(cmd: impl Into<String,>, args: I,) -> LlccB<ExitStatus,>
where
	I: IntoIterator<Item = S,>,
	S: AsRef<OsStr,>,
{
	let status = Command::new(cmd.into(),).args(args,).status()?;
	X(status,)
}

pub fn clear_out(dest: &Dest,) -> LlccB<(),> {
	let out = dest.path(DestKind::OutDir,).into();
	if fs::exists(&out,)? {
		fs::remove_dir_all(out,)?;
	}

	X((),)
}
