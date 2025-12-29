use llcc_error::LlccB;

use crate::Ctx;
use std::marker::PhantomData;
use std::ops::FromResidual;
use std::ops::Try;

pub trait State {
	type Data;
	fn update(&mut self,);
}

pub trait DefState<BuildTarget: State,> {
	fn into_state(self,) -> BuildTarget;
}

pub trait Worker {
	type Ctx: Ctx;
	type State: State;
	type Rslt: FromResidual + Try;
	fn work(state: &Self::State, ctx: &Self::Ctx,) -> Self::Rslt;
}

pub trait DefWorker<BuildTarget: Worker,> {
	fn into_worker(self,) -> BuildTarget;
}

pub trait Layer: State {
	type NextState: State;
	type NextLayer: Layer;
	type Converter: Worker;

	fn next_layer(&self,) -> LlccB<Option<Self::NextLayer,>,>;
	fn next_state(&self,) -> LlccB<Self::NextState,>;
}

pub struct CoreLayer<
	S: State,
	C: Ctx,
	NextState: State,
	NextLayer: Layer,
	Converter: Worker,
> {
	s:   S,
	ctx: C,
	__:  PhantomData<(NextState, NextLayer, Converter,),>,
}

impl<S: State, C: Ctx, NextState: State, NextLayer: Layer, Converter: Worker,>
	Layer for CoreLayer<S, C, NextState, NextLayer, Converter,>
{
	type Converter = Converter;
	type NextLayer = NextLayer;
	type NextState = NextState;

	fn next_layer(&self,) -> LlccB<Option<Self::NextLayer,>,> {
		todo!()
	}

	fn next_state(&self,) -> LlccB<Self::NextState,> {
		todo!()
	}
}

impl<S: State, C: Ctx, NextState: State, NextLayer: Layer, Converter: Worker,>
	State for CoreLayer<S, C, NextState, NextLayer, Converter,>
{
	type Data = S;

	fn update(&mut self,) {
		todo!()
	}
}
