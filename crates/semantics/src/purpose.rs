use crate::Ctx;
use llcc_error::LlccB;
use std::marker::PhantomData;

pub trait State {}

pub trait InclementalState {
	//  TODO: ctxをctxとdeltaに分割した方が良い?
	type Ctx;
	fn inclement(&mut self, ctx: &Self::Ctx,) -> LlccB<(),>;
}

pub trait Worker<I, Ctx,> {
	type Output;

	fn work(&self, input: I, ctx: &Ctx,) -> LlccB<Self::Output,>;
}

impl<I, Ctx, F, O,> Worker<I, Ctx,> for F
where F: Fn(I, &Ctx,) -> LlccB<O,>
{
	type Output = O;

	fn work(&self, input: I, ctx: &Ctx,) -> LlccB<Self::Output,> {
		self.call((input, ctx,),)
	}
}

pub trait Lowering:
	for<'a> Worker<&'a <Self::Upper as Layer>::State, <Self::Upper as Layer>::Ctx,>
{
	type Upper: Layer;
	fn lower<'a,>(
		&self,
		upper: &'a Self::Upper,
	) -> LlccB<
		<Self as Worker<
			&'a <Self::Upper as Layer>::State,
			<Self::Upper as Layer>::Ctx,
		>>::Output,
	> {
		self.work(upper.state(), upper.ctx(),)
	}
}

pub trait Layer {
	type State: State;
	type Ctx: Ctx;
	type Worker: for<'a> Worker<&'a Self::State, Self::Ctx,>;

	fn state(&self,) -> &Self::State;
	fn state_mut(&mut self,) -> &mut Self::State;
	fn ctx(&self,) -> &Self::Ctx;
	fn ctx_mut(&mut self,) -> &mut Self::Ctx;

	fn apply_work(
		&self,
		worker: Self::Worker,
	) -> LlccB<<Self::Worker as Worker<&Self::State, Self::Ctx,>>::Output,> {
		worker.work(self.state(), self.ctx(),)
	}
}

pub trait CanonicalForm: State {
	type Ctx: Ctx;
	fn verify_canonical(&self,) -> LlccB<(),>;
}

/// Internal Representationと言われる物
pub trait FrontEnd: Layer
where <Self as Layer>::State: CanonicalForm
{
}

pub struct CoreLayer<S, C, W,>
where
	S: State,
	C: Ctx,
	W: for<'a> Worker<&'a S, C,>,
{
	state:   S,
	ctx:     C,
	_worker: PhantomData<W,>,
}

impl<S, C, W,> Layer for CoreLayer<S, C, W,>
where
	S: State,
	C: Ctx,
	W: for<'a> Worker<&'a S, C,>,
{
	type Ctx = C;
	type State = S;
	type Worker = W;

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
}

impl<S, C, W,> CoreLayer<S, C, W,>
where
	S: State,
	C: Ctx,
	W: for<'a> Worker<&'a S, C,>,
{
	pub fn new(state: S, ctx: C,) -> Self {
		Self { state, ctx, _worker: PhantomData::<W,>, }
	}
}
