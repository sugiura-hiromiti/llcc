use std::convert::Infallible;
use std::fmt::Debug;
use std::io;
use std::ops::ControlFlow;
use std::ops::FromResidual;
use std::ops::Try;
use std::panic::Location;
use std::process::Termination;

pub type LlccB<S,> = B<S, LlccError,>;

/// b stands for boolish, branch and binary
pub enum B<S, T,> {
	X(S,),
	Y(T,),
}

impl<S, T,> FromResidual for B<S, T,> {
	fn from_residual(residual: <Self as std::ops::Try>::Residual,) -> Self {
		match residual {
			B::X(_i,) => unreachable!(),
			B::Y(t,) => Self::Y(t,),
		}
	}
}

impl<S, T: From<E,>, E: std::error::Error,> FromResidual<Result<Infallible, E,>,>
	for B<S, T,>
{
	fn from_residual(residual: Result<Infallible, E,>,) -> Self {
		match residual {
			Ok(_i,) => unreachable!(),
			Err(e,) => Self::Y(T::from(e,),),
		}
	}
}

impl<S, T,> Try for B<S, T,> {
	type Output = S;
	type Residual = B<Infallible, T,>;

	fn from_output(output: Self::Output,) -> Self {
		Self::X(output,)
	}

	fn branch(self,) -> std::ops::ControlFlow<Self::Residual, Self::Output,> {
		match self {
			Self::X(s,) => ControlFlow::Continue(s,),
			Self::Y(t,) => ControlFlow::Break(B::Y(t,),),
		}
	}
}

impl<S, T,> Termination for B<S, T,> {
	fn report(self,) -> std::process::ExitCode {
		match self {
			Self::X(_,) => std::process::ExitCode::SUCCESS,
			Self::Y(_,) => std::process::ExitCode::FAILURE,
		}
	}
}

pub trait ReShape<O, C,> {
	fn reshape(self, ctx: C,) -> O;
}

// impl<T, E: From<C,>, C,> ReShape<Result<T, E,>, C,> for Option<T,> {
// 	fn reshape(self, ctx: C,) -> Result<T, E,> {
// 		match self {
// 			Self::Some(t,) => Ok(t,),
// 			Self::None => Err(E::from(ctx,),),
// 		}
// 	}
// }

impl<T, E,> ReShape<B<T, E,>, (),> for Result<T, E,> {
	fn reshape(self, _ctx: (),) -> B<T, E,> {
		match self {
			Self::Ok(t,) => B::X(t,),
			Self::Err(e,) => B::Y(e,),
		}
	}
}

impl<T, E: From<C,>, C,> ReShape<B<T, E,>, C,> for Option<T,> {
	fn reshape(self, ctx: C,) -> B<T, E,> {
		match self {
			Self::Some(t,) => B::X(t,),
			Self::None => B::Y(E::from(ctx,),),
		}
	}
}

#[derive(thiserror::Error, Debug,)]
pub enum LlccError<E = String,>
where E: Debug
{
	#[error("io error at {loc}: {0}", loc= Location::caller())]
	Io(#[from] io::Error,),
	#[error("unknown error happen at {loc}: {0}",loc=Location::caller())]
	Unknowwn(E,),
}

impl From<&str,> for LlccError {
	fn from(value: &str,) -> Self {
		Self::Unknowwn(value.to_string(),)
	}
}
