use crate::semantic_core::SemanticCore;
use llcc_error::LlccB;
use llcc_semantics::Ctx;
use llcc_semantics::purpose::CoreLayer;
use llcc_semantics::purpose::State;
use llcc_semantics::purpose::Worker;

pub type Parser = CoreLayer<Ast, ParserCtx, ParserWorker,>;

pub struct Ast {
	root: Node,
}

impl State for Ast {
	type Ctx = ParserCtx;

	fn inclement(&mut self, ctx: &Self::Ctx,) -> LlccB<(),> {
		todo!()
	}
}

pub enum Node {
	Dummy(String,),
}

#[derive(Default,)]
pub struct ParserCtx;

impl Ctx for ParserCtx {
	const ROLE: &'static str = "parser ctx";
}

pub struct ParserWorker;

impl Worker<&Ast, ParserCtx,> for ParserWorker {
	type Output = SemanticCore;

	fn work(&self, input: &Ast, ctx: &ParserCtx,) -> LlccB<Self::Output,> {
		let _ = input;
		let _ = ctx;
		todo!()
	}
}
