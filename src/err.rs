use std::convert::Infallible;
use std::fmt::Debug;
use std::fmt::Display;
use std::io;
use std::num;
use std::ops::ControlFlow;
use std::ops::FromResidual;
use std::ops::Try;
use std::panic::Location;
use std::process::Termination;

#[cfg(test)] use quickcheck::Testable;

pub type LlccB<S,> = B<S, LlccError,>;

/// b stands for boolish, branch and binary
pub enum B<S, T,> {
	X(S,),
	Y(T,),
}

impl<S, T,> FromResidual for B<S, T,> {
	#[track_caller]
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
	#[track_caller]
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

impl<S, T: Display,> Termination for B<S, T,> {
	fn report(self,) -> std::process::ExitCode {
		match self {
			Self::X(_,) => std::process::ExitCode::SUCCESS,
			Self::Y(t,) => {
				eprintln!("{t:#}");
				std::process::ExitCode::FAILURE
			},
		}
	}
}

pub trait ReShape<O, C,> {
	fn reshape(self, ctx: C,) -> O;
}

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

#[cfg(test)]
impl<S: 'static, T: 'static + Debug,> Testable for B<S, T,> {
	fn result(&self, _: &mut quickcheck::Gen,) -> quickcheck::TestResult {
		use quickcheck::TestResult;

		match self {
			Self::X(_,) => TestResult::from_bool(true,),
			Self::Y(t,) => TestResult::error(format!("{t:#?}"),),
		}
	}
}

#[derive(Debug,)]
pub enum LlccError<E = String,>
where E: Debug
{
	Io { source: io::Error, loc: &'static Location<'static,>, },
	Parse { source: num::ParseIntError, loc: &'static Location<'static,>, },
	Unknown(E,),
}

impl Display for LlccError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_,>,) -> std::fmt::Result {
		match self {
			Self::Io { source, loc, } => {
				f.write_fmt(format_args!("{source} at: [{}]", loc),)
			},
			Self::Parse { source, loc, } => {
				f.write_fmt(format_args!("{source} at: [{}]", loc),)
			},
			Self::Unknown(e,) => f.write_fmt(format_args!("{e}"),),
		}
	}
}

impl std::error::Error for LlccError {}

impl From<io::Error,> for LlccError {
	#[track_caller]
	fn from(value: io::Error,) -> Self {
		LlccError::Io { source: value, loc: Location::caller(), }
	}
}

impl From<num::ParseIntError,> for LlccError {
	#[track_caller]
	fn from(value: num::ParseIntError,) -> Self {
		LlccError::Parse { source: value, loc: Location::caller(), }
	}
}

impl From<&str,> for LlccError {
	#[track_caller]
	fn from(value: &str,) -> Self {
		Self::Unknown(format!(
			"unclassified error: {value}\nat: [{}]",
			Location::caller()
		),)
	}
}
