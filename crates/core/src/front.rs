use crate::LlccB;
use crate::asm::run_cmd;
use llcc_error::B::X;
use llcc_error::ReShape;
use std::path::PathBuf;
use std::process::ExitStatus;

// struct CompileCtx<'a,> {
// 	src:      &'a str,
// 	out_path: &'a Path,
// }
//
// struct ParseCtx<'a,> {
// 	src:      &'a str,
// 	out_path: Option<&'a Path,>,
// }
//
// impl<'a,> CompileCtx<'a,> {
// 	fn to_parse_ctx<const WRITE_FILE: bool,>(&self,) -> ParseCtx<'a,> {
// 		ParseCtx {
// 			src:      self.src,
// 			out_path: if WRITE_FILE { Some(self.out_path,) } else { None },
// 		}
// 	}
// }

// impl<C: CompileCtx,> Compiler<C,> for LlccCompiler {}
pub struct Tokenizer<'a,> {
	src: &'a str,
}

pub struct TokenStream<'a,>(Vec<Token<'a,>,>,);

pub enum Token<'a,> {
	Ident(&'a str,),
}

pub struct Parser<'a,> {
	src: &'a str,
}

pub struct SemanticCore<'a,> {
	ast: Ast<'a,>,
}

pub struct Ast<'a,> {
	root: Node<'a,>,
}

pub enum Node<'a,> {
	Dummy(&'a str,),
}

pub struct Compiler {}

// impl Compiler {
// 	/// # Return
// 	///
// 	/// returns path to generated executable file
// 	pub fn compile(&self, src: &str,) -> LlccB<impl Into<PathBuf,>,> {
// 		self.emit_asm(src,)?;
// 		self.assemble()?;
// 		self.link()?;
// 		todo!()
// 		// X(self.dest.path(DestKind::Exe,),)
// 	}
//
// 	/// # Return
// 	///
// 	/// returns path to generated assembly file
// 	pub fn emit_asm(
// 		&self,
// 		src: impl Into<String,>,
// 	) -> LlccB<impl Into<PathBuf,>,> {
// 		let asm = asm_str(src,)?;
// 		todo!()
// 		// write_asm(asm, self.dest.path(DestKind::Asm,),)?;
// 		// X(self.dest.path(DestKind::Asm,),)
// 	}
//
// 	/// # Return
// 	///
// 	/// returns path to generated object file
// 	pub fn assemble(&self,) -> LlccB<impl Into<PathBuf,>,> {
// 		todo!()
// 		// let obj_path = stringify_path(self.dest.path(DestKind::Obj,),)?;
// 		// let asm_path = stringify_path(self.dest.path(DestKind::Asm,),)?;
//
// 		// run_cmd("as", ["-o", &obj_path, &asm_path,],)?;
// 		// X(obj_path,)
// 	}
//
// 	/// # Return
// 	///
// 	/// returns path to generated executable file
// 	pub fn link(&self,) -> LlccB<impl Into<PathBuf,>,> {
// 		todo!()
// 		// let exe_path = stringify_path(self.dest.path(DestKind::Exe,),)?;
// 		// let obj_path = stringify_path(self.dest.path(DestKind::Obj,),)?;
// 		//
// 		// run_cmd("ld", ["-o", &exe_path, &obj_path,],)?;
// 		// X(exe_path,)
// 	}
// }

pub fn exec(exe_path: impl Into<PathBuf,>,) -> LlccB<ExitStatus,> {
	run_cmd::<[&str; 0], &str,>(
		exe_path.into().to_str().reshape("failed to stringify exe_path",)?,
		[],
	)
}
