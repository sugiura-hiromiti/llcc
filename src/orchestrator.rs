use crate::LlccB;
use crate::err::B::X;
use crate::err::B::Y;
use crate::err::LlccError;
use crate::front::LlccCompiler;
use crate::front::exec;
use crate::semantics::Ctx;
use crate::semantics::RunCtx;
use crate::semantics::SrcCtx;
use crate::semantics::context::HasIn;
use crate::semantics::context::HasOut;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;
use std::process::ExitStatus;

mod file_manage;

pub enum Src<'a,> {
	Str(&'a str,),
	Path(&'a Path,),
}

pub trait Own {
	type Owned;
	fn own(&self,) -> Self::Owned;
}

pub enum SrcOwned {
	Str(String,),
	Path(PathBuf,),
}

impl<'a,> Own for Src<'a,> {
	type Owned = SrcOwned;

	fn own(&self,) -> Self::Owned {
		match self {
			Self::Str(s,) => SrcOwned::Str(s.to_string(),),
			Self::Path(path,) => SrcOwned::Path(path.to_path_buf(),),
		}
	}
}

/// this is minimal example of RunCtx implementation
pub struct MockRunMeta<'a,> {
	src:   Src<'a,>,
	files: file_manage::Dest,
}

impl<'a,> SrcCtx for MockRunMeta<'a,> {}
impl<'a,> RunCtx for MockRunMeta<'a,> {}

impl<'a,> HasIn for MockRunMeta<'a,> {
	type InInfo = SrcOwned;
	type InInfoRef<'i,>
		= &'i Src<'i,>
	where Self: 'i;

	fn in_info(&self,) -> Option<Self::InInfoRef<'_,>,> {
		Some(&self.src,)
	}

	fn in_info_owned(&self,) -> Option<Self::InInfo,> {
		Some(self.src.own(),)
	}
}

impl<'a,> HasOut for MockRunMeta<'a,> {
	type OutInfo = file_manage::Dest;
	type OutInfoRef<'o,>
		= &'o file_manage::Dest
	where Self: 'o;

	fn out_info(&self,) -> Option<Self::OutInfoRef<'_,>,> {
		Some(&self.files,)
	}

	fn out_info_owned(&self,) -> Option<Self::OutInfo,> {
		todo!()
	}
}

impl<'a,> Ctx for MockRunMeta<'a,> {
	const ROLE: &'static str = "mock orchestration";
}

pub fn post_process<R: SrcCtx,>(ctx: R,) -> LlccB<(String, LlccCompiler,),>
where R: HasIn<InInfo = SrcOwned,> {
	let Some(src,) = ctx.in_info_owned() else {
		return Y(LlccError::lack_of_ctx::<R,>(),);
	};

	let src = match src {
		SrcOwned::Str(s,) => s,
		SrcOwned::Path(path,) => {
			let mut input = File::open(path,)?;

			let mut buf = String::new();
			input.read_to_string(&mut buf,)?;
			buf
		},
	};

	X((src, LlccCompiler::default(),),)
}

pub fn run<RC,>(ctx: RC,) -> LlccB<ExitStatus,>
where
	RC: RunCtx + SrcCtx,
	RC: HasIn<InInfo = SrcOwned,>,
{
	//  TODO: unwrap_or/unwrap_or_defaultにする
	let (src, compiler,) = post_process(ctx,)?;
	let exe_path = compiler.compile(src,)?;
	exec(exe_path,)
}

#[cfg(test)]
pub fn run_fixture(src: &str,) -> LlccB<MockRunMeta<'_,>,> {
	use uuid::Uuid;

	X(MockRunMeta {
		src:   Src::Str(src,),
		files: file_manage::Dest::new(
			Some("test".to_string(),),
			Uuid::new_v4(),
			None,
		)?,
	},)
}
#[cfg(test)]
mod tests {
	use super::*;
	use quickcheck_macros::quickcheck;

	#[quickcheck]
	fn test_run_single_number(es: u8,) -> LlccB<(),> {
		let es_str = es.to_string();
		let es_str = es_str.as_str();
		let exit_status = run(run_fixture(es_str,)?,)?;
		assert_eq!(exit_status.code(), Some(es as i32));
		X((),)
	}
}
