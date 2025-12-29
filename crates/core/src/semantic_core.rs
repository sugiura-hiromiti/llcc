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
	type Next = Self;
	type State = Self;
	type Worker = Self;

	fn from_state(state: Self::State,) -> Self {
		let _ = state;
		todo!()
	}

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

	fn set_worker(&mut self, worker: Self::Worker,) -> &mut Self {
		let _ = worker;
		todo!()
	}

	fn next(&self,) -> LlccB<Self::Next,> {
		todo!()
	}
}

impl State for SemanticCore {
	type Ctx = ();

	fn update(&mut self, ctx: &Self::Ctx,) -> LlccB<(),> {
		let _ = ctx;
		todo!()
	}
}

impl Ctx for SemanticCore {
	const ROLE: &'static str = "semantic core";
}

impl<I, Ctx,> Worker<I, Ctx,> for SemanticCore {
	type Output = Self;

	fn work(&self, _input: I, _ctx: &Ctx,) -> LlccB<Self::Output,> {
		unimplemented!()
	}
}
