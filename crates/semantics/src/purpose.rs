use llcc_error::B::X;
use llcc_error::LlccB;
use llcc_error::LlccError;
use llcc_error::ReShape;

use crate::Ctx;
use std::marker::PhantomData;
use std::panic::Location;

pub trait State {
	type Ctx;
	fn update(&mut self, ctx: &Self::Ctx,) -> LlccB<(),>;
}

// pub trait DefState<BuildTarget: State,> {
// 	fn into_state(self,) -> BuildTarget;
// }

pub trait Worker<I, Ctx,> {
	type Output;

	fn work(&self, input: I, ctx: &Ctx,) -> LlccB<Self::Output,>;
}

// impl<I, Ctx, O, T: for<'a> Fn(I, &'a Ctx,) -> LlccB<O,>,> Worker<I, Ctx,>
// 	for T
// {
// 	type Output = O;
//
// 	fn work(&self, input: I, ctx: &Ctx,) -> LlccB<Self::Output,> {
// 		self.call((input, ctx,),)
// 	}
// }

// impl<I, Ctx, W: Worker<I, Ctx,>,> Worker<I, Ctx,> for Box<W,> {
// 	type Output = W::Output;
//
// 	fn work(&self, input: I, ctx: &Ctx,) -> LlccB<Self::Output,> {
// 		self.as_ref().work(input, ctx,)
// 	}
// }

// pub trait DefWorker<BuildTarget: Worker,> {
// 	fn into_worker(self,) -> BuildTarget;
// }

pub trait Layer {
	type State: State;
	type Next: Layer;
	type Ctx: Ctx;
	type Worker: for<'a> Worker<
			&'a Self::State,
			Self::Ctx,
			Output = <Self::Next as Layer>::State,
		>;

	fn from_state(state: Self::State,) -> Self;
	fn state(&self,) -> &Self::State;
	fn state_mut(&mut self,) -> &mut Self::State;
	fn ctx(&self,) -> &Self::Ctx;
	fn ctx_mut(&mut self,) -> &mut Self::Ctx;
	fn set_worker(&mut self, worker: Self::Worker,) -> &mut Self;

	// fn update(&mut self,) -> LlccB<&mut Self,>;
	fn next(&self,) -> LlccB<Self::Next,>;
}

pub trait LayerErr {
	#[track_caller]
	fn layer_has_no_worker(msg: &str,) -> LlccError {
		LlccError::LayerHasNoWorker {
			msg: msg.to_string(),
			loc: Location::caller(),
		}
	}
}

impl LayerErr for LlccError {}

pub struct CoreLayer<S, C, W, N,>
where
	S: State,
	C: Ctx,
	W: for<'a> Worker<&'a S, C, Output = N::State,>,
	N: Layer,
{
	state:  S,
	ctx:    C,
	worker: Option<W,>,
	_next:  PhantomData<N,>,
}

impl<S, C, W, N,> Layer for CoreLayer<S, C, W, N,>
where
	S: State,
	C: Ctx,
	W: for<'a> Worker<&'a S, C, Output = N::State,>,
	N: Layer,
{
	type Ctx = C;
	type Next = N;
	type State = S;
	type Worker = W;

	fn from_state(state: Self::State,) -> Self {
		Self {
			state,
			ctx: Self::Ctx::default(),
			worker: None,
			_next: PhantomData::<Self::Next,>,
		}
	}

	fn state(&self,) -> &Self::State {
		&self.state
	}

	fn state_mut(&mut self,) -> &mut Self::State {
		&mut self.state
	}

	fn ctx(&self,) -> &Self::Ctx {
		&self.ctx
	}

	fn ctx_mut(&mut self,) -> &mut Self::Ctx {
		&mut self.ctx
	}

	fn set_worker(&mut self, worker: Self::Worker,) -> &mut Self {
		self.worker = Some(worker,);
		self
	}

	// fn update(&mut self,) -> LlccB<&mut Self,> {
	// 	self.state_mut().update(self.ctx(),)?;
	// 	X(self,)
	// }

	fn next(&self,) -> LlccB<Self::Next,> {
		let next_state = self
			.worker
			.as_ref()
			.reshape(
				LlccError::layer_has_no_worker("CoreLayer has no worker",),
			)?
			.work(&self.state, &self.ctx,)?;
		X(Self::Next::from_state(next_state,),)
	}
}
