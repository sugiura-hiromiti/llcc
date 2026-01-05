use llcc_error::LlccB;
use llcc_semantics::Ctx;
use llcc_semantics::purpose::Layer;
use llcc_semantics::purpose::State;
use llcc_semantics::purpose::Worker;

pub struct SemanticCore {}

impl Default for SemanticCore {
	fn default() -> Self {
		Self {}
	}
}

impl Layer for SemanticCore {
	type Ctx = Self;
	type State = Self;
	type Worker = Self;

	fn state(&self,) -> &Self::State {
		todo!()
	}

	fn state_mut(&mut self,) -> &mut Self::State {
		todo!()
	}

	fn ctx(&self,) -> &Self::Ctx {
		todo!()
	}

	fn ctx_mut(&mut self,) -> &mut Self::Ctx {
		todo!()
	}
}

impl State for SemanticCore {}

impl Ctx for SemanticCore {
	const ROLE: &'static str = "semantic core";
}

impl<I, Ctx,> Worker<I, Ctx,> for SemanticCore {
	type Output = Self;

	fn work(&self, _input: I, _ctx: &Ctx,) -> LlccB<Self::Output,> {
		unimplemented!()
	}
}
