use crate::LlccB;
use crate::asm::run_cmd;
use llcc_error::ReShape;
use std::path::PathBuf;
use std::process::ExitStatus;

pub fn exec(exe_path: impl Into<PathBuf,>,) -> LlccB<ExitStatus,> {
	run_cmd::<[&str; 0], &str,>(
		exe_path.into().to_str().reshape("failed to stringify exe_path",)?,
		[],
	)
}

pub trait SrcRef {
	fn source_code(&self,) -> String;
}
