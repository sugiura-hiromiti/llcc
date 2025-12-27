use crate::semantics::Ctx;
#[cfg(test)] use quickcheck::Testable;
use std::any::type_name;
use std::convert::Infallible;
use std::fmt::Debug;
use std::fmt::Display;
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

impl<T, E,> ReShape<Result<T, E,>, (),> for B<T, E,> {
	fn reshape(self, _ctx: (),) -> Result<T, E,> {
		match self {
			Self::X(t,) => Ok(t,),
			Self::Y(e,) => Err(e,),
		}
	}
}
impl<T, E,> ReShape<Option<T,>, (),> for B<T, E,> {
	fn reshape(self, _ctx: (),) -> Option<T,> {
		match self {
			Self::X(t,) => Some(t,),
			Self::Y(_,) => None,
		}
	}
}

pub trait Container {
	type T;
	fn unwrap(self,) -> Self::T;
	fn expect(self, msg: &str,) -> Self::T;
}

impl<T, E: std::fmt::Debug,> Container for B<T, E,> {
	type T = T;

	fn unwrap(self,) -> Self::T {
		let a: Result<_, _,> = self.reshape((),);
		a.unwrap()
	}

	fn expect(self, msg: &str,) -> Self::T {
		let a: Result<_, _,> = self.reshape((),);
		a.expect(msg,)
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
pub enum LlccError {
	Io {
		source: io::Error,
		loc:    &'static Location<'static,>,
	},
	ParseSrcInt {
		source: std::num::ParseIntError,
		loc:    &'static Location<'static,>,
	},
	Parse {
		source: strum::ParseError,
		loc:    &'static Location<'static,>,
	},
	MismatchImmediateType {
		origin:    i32,
		max_bit:   u8,
		is_signed: bool,
		loc:       &'static Location<'static,>,
	},
	LackOfContext {
		context_role: &'static str,
		type_name:    &'static str,
		loc:          &'static Location<'static,>,
	},
	Unknown {
		msg: String,
		loc: &'static Location<'static,>,
	},
}

impl LlccError {
	#[track_caller]
	pub fn mismatch_imm(origin: i32, max_bit: u8, is_signed: bool,) -> Self {
		LlccError::MismatchImmediateType {
			origin,
			max_bit,
			is_signed,
			loc: Location::caller(),
		}
	}

	#[track_caller]
	pub fn lack_of_ctx<C: Ctx,>() -> Self {
		LlccError::LackOfContext {
			context_role: C::ROLE,
			type_name:    type_name::<C,>(),
			loc:          Location::caller(),
		}
	}
}

impl Display for LlccError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_,>,) -> std::fmt::Result {
		match self {
			Self::Io { source, loc, } => {
				f.write_fmt(format_args!("{source} at: [{loc}]"),)
			},
			Self::ParseSrcInt { source, loc, } => {
				f.write_fmt(format_args!("{source} at: [{loc}]"),)
			},
			Self::Parse { source, loc, } => {
				f.write_fmt(format_args!("{source} at: [{loc}]",),)
			},
			Self::MismatchImmediateType {
				origin, max_bit, is_signed, loc,
			} => f.write_fmt(format_args!(
				"{origin} is not of type {max_bit} bit {} int. at: [{loc}]",
				if *is_signed { "signed" } else { "unsigned" }
			),),
			Self::LackOfContext { context_role, type_name, loc, } => f
				.write_fmt(format_args!(
					"context: `{type_name}` for {context_role} should take \
					 enough info. at: [{loc}]"
				),),
			Self::Unknown { msg, loc, } => {
				f.write_fmt(format_args!("{msg} at: [{loc}]"),)
			},
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

impl From<std::num::ParseIntError,> for LlccError {
	#[track_caller]
	fn from(value: std::num::ParseIntError,) -> Self {
		LlccError::ParseSrcInt { source: value, loc: Location::caller(), }
	}
}

impl From<strum::ParseError,> for LlccError {
	#[track_caller]
	fn from(value: strum::ParseError,) -> Self {
		LlccError::Parse { source: value, loc: Location::caller(), }
	}
}

impl From<&str,> for LlccError {
	#[track_caller]
	fn from(value: &str,) -> Self {
		Self::Unknown { msg: value.to_string(), loc: Location::caller(), }
	}
}
