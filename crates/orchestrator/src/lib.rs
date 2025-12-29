use llcc_error::B::X;
use llcc_error::LlccB;
use std::fs;
use std::io::Read as _;
use std::path::Path;
use std::process::ExitStatus;

mod file_manage;

pub enum Src<'a,> {
	Str(&'a str,),
	Path(&'a Path,),
}

impl<'a,> Src<'a,> {
	fn get_code(self,) -> LlccB<String,> {
		let code = match self {
			Self::Str(s,) => s.to_string(),
			Self::Path(path,) => {
				let mut input = fs::File::open(path,)?;
				let mut buf = String::new();
				input.read_to_string(&mut buf,)?;
				buf
			},
		};
		X(code,)
	}
}

pub fn run(src: Src<'_,>,) -> LlccB<ExitStatus,> {
	let _code = src.get_code()?;
	todo!()
	// let exe_path = compiler.compile(src,)?;
	// exec(exe_path,)
}

pub fn clear_out(dest: &file_manage::Dest,) -> LlccB<(),> {
	let out = dest.path(file_manage::DestKind::OutDir,).into();
	if fs::exists(&out,)? {
		fs::remove_dir_all(out,)?;
	}

	X((),)
}

#[cfg(test)]
mod tests {
	use super::*;
	use quickcheck_macros::quickcheck;

	#[quickcheck]
	fn test_run_single_number(es: u8,) -> LlccB<(),> {
		let es_str = es.to_string();
		let es_str = es_str.as_str();
		let exit_status = run(Src::Str(es_str,),)?;
		assert_eq!(exit_status.code(), Some(es as i32));
		X((),)
	}
}
