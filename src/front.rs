use crate::LlccB;
use crate::asm::asm_str;
use crate::asm::run_cmd;
use crate::asm::write_asm;
use crate::err::B::X;
use crate::err::ReShape;
use crate::file_io::Dest;
use crate::file_io::DestKind;
use crate::stringify_path;
use std::fs;
use std::io::Read;
use std::path::PathBuf;
use std::process::ExitStatus;

pub fn post_process<S: Into<String,>,>(
	src: Option<S,>,
) -> LlccB<(String, Compiler,),> {
	let compiler = Compiler::default();
	if let Some(src,) = src {
		return X((src.into(), compiler,),);
	}

	let src_path = dbg!(compiler.src_path().into());
	let mut input = fs::File::open(compiler.src_path().into(),)?;
	let mut buf = String::new();
	input.read_to_string(&mut buf,)?;
	let buf = buf.trim_end().to_string();
	X((buf, compiler,),)
}

pub fn run(src: Option<impl Into<String,>,>,) -> LlccB<ExitStatus,> {
	let (src, compiler,) = post_process(src,)?;
	let exe_path = compiler.compile(src,)?;
	exec(exe_path,)
}

#[derive(Default,)]
pub struct Compiler {
	dest: Dest,
}

impl Compiler {
	fn src_path(&self,) -> impl Into<PathBuf,> {
		self.dest.path(DestKind::Src,)
	}

	/// # Return
	///
	/// returns path to generated executable file
	pub fn compile(
		&self,
		src: impl Into<String,>,
	) -> LlccB<impl Into<PathBuf,>,> {
		self.emit_asm(src,)?;
		self.assemble()?;
		self.link()?;
		X(self.dest.path(DestKind::Exe,),)
	}

	/// # Return
	///
	/// returns path to generated assembly file
	pub fn emit_asm(
		&self,
		src: impl Into<String,>,
	) -> LlccB<impl Into<PathBuf,>,> {
		let asm = asm_str(src,)?;
		write_asm(asm, self.dest.path(DestKind::Asm,),)?;
		X(self.dest.path(DestKind::Asm,),)
	}

	/// # Return
	///
	/// returns path to generated object file
	pub fn assemble(&self,) -> LlccB<impl Into<PathBuf,>,> {
		let obj_path = stringify_path(self.dest.path(DestKind::Obj,),)?;
		let asm_path = stringify_path(self.dest.path(DestKind::Asm,),)?;

		run_cmd("as", ["-o", &obj_path, &asm_path,],)?;
		X(obj_path,)
	}

	/// # Return
	///
	/// returns path to generated executable file
	pub fn link(&self,) -> LlccB<impl Into<PathBuf,>,> {
		let exe_path = stringify_path(self.dest.path(DestKind::Exe,),)?;
		let obj_path = stringify_path(self.dest.path(DestKind::Obj,),)?;

		run_cmd("ld", ["-o", &exe_path, &obj_path,],)?;
		X(exe_path,)
	}
}

pub fn exec(exe_path: impl Into<PathBuf,>,) -> LlccB<ExitStatus,> {
	run_cmd::<[&str; 0], &str,>(
		exe_path.into().to_str().reshape("failed to stringify exe_path",)?,
		[],
	)
}

#[cfg(test)]
mod tests {
	use super::*;
	use quickcheck_macros::quickcheck;

	#[quickcheck]
	fn test_run(es: u8,) -> LlccB<(),> {
		let exit_status = run(Some(es.to_string(),),)?;
		assert_eq!(exit_status.code(), Some(es as i32));
		X((),)
	}
}
