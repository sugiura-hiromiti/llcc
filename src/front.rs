use crate::LlccB;
use crate::asm::asm_str;
use crate::asm::run_cmd;
use crate::asm::write_asm;
use crate::err::B::X;
use crate::err::ReShape;
use crate::semantics::Convert;
use crate::semantics::context::HasIn;
use crate::semantics::context::HasOut;
use crate::semantics::purpose::Layer;
// use crate::file_io::Dest;
// use crate::file_io::DestKind;
use crate::stringify_path;
use std::path::PathBuf;
use std::process::ExitStatus;

#[derive(Default,)]
pub struct LlccCompiler {}

impl LlccCompiler {
	#[deprecated(note = "入力はオーケストレーション層の管理領域")]
	pub fn src_path(&self,) -> impl Into<PathBuf,> {
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
