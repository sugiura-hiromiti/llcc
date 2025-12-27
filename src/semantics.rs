use crate::err::LlccB;
use crate::semantics::ability as a;
use crate::semantics::context as c;

pub mod ability;
pub mod context;
pub mod purpose;

/// 表現変換
pub trait Convert<LayerFrom, LayerTo,const REVERSIBLE: > {
	fn convert(&self,);
}

pub trait Orchestrate {}

pub trait Runner: c::HasIn + a::ReadIn + a::Exec {
	fn run(&self,) -> LlccB<impl RunState,>;
}

pub trait RunState {
	type State;
	fn state(&self,) -> Self::State;
}

pub trait Ctx {
	/// description of context role
	const ROLE: &'static str;
}

pub trait RunCtx: Ctx + c::HasOut + c::HasIn {
	fn as_run_ctx(&self,) -> &impl RunCtx
	where Self: Sized {
		self
	}
}

pub trait SrcCtx: Ctx + c::HasOut + c::HasIn {
	fn as_src_ctx(&self,) -> &impl SrcCtx
	where Self: Sized {
		self
	}
}

pub trait CompileCtx: Ctx + c::HasOut + c::HasIn {}

pub trait Compiler<C: CompileCtx,> {}
