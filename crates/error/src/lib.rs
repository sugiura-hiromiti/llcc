#![feature(try_trait_v2)]
#![feature(try_trait_v2_residual)]

use quickcheck::Testable;
use std::convert::Infallible;
use std::fmt::Debug;
use std::fmt::Display;
use std::io;
use std::ops::ControlFlow;
use std::ops::FromResidual;
use std::ops::Residual;
use std::ops::Try;
use std::panic::Location;
use std::process::Termination;
use strum::Display;

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

impl<S, T,> Residual<S,> for B<Infallible, T,> {
	type TryType = B<S, T,>;
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
pub struct LlccError {
	kind: LlccErrorKind,
	loc:  &'static Location<'static,>,
}

#[derive(Debug,)]
enum LlccErrorKind {
	Io(io::Error,),
	ParseSrcInt(std::num::ParseIntError,),
	Parse(strum::ParseError,),
	ImmediateType(ImmTypeError,),
	Context(ContextError,),
	Layer(LayerError,),
	Token(TokenError,),
	Unknown(String,),
}

#[derive(Debug,)]
pub enum TokenError {
	UnexpectedToken,
}

#[derive(Debug,)]
pub struct ImmTypeError {
	origin:    i32,
	max_bit:   u8,
	is_signed: bool,
}

impl ImmTypeError {
	pub fn new(origin: i32, max_bit: u8, is_signed: bool,) -> Self {
		Self { origin, max_bit, is_signed, }
	}
}

#[derive(Debug,)]
pub struct ContextError {
	context_role: &'static str,
	type_name:    &'static str,
}

#[derive(Debug,)]
pub enum LayerError {
	LayerHasNoWorker,
}

impl Display for LlccError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_,>,) -> std::fmt::Result {
		let loc = self.loc;
		match &self.kind {
			LlccErrorKind::Io(error,) => {
				f.write_fmt(format_args!("{error:?}\nat: [{loc}]"),)
			},
			LlccErrorKind::ParseSrcInt(error,) => {
				f.write_fmt(format_args!("{error:?}\nat: [{loc}]"),)
			},
			LlccErrorKind::Parse(error,) => {
				f.write_fmt(format_args!("{error:?}\nat: [{loc}]"),)
			},
			LlccErrorKind::ImmediateType(error,) => {
				f.write_fmt(format_args!("{error:?}\nat: [{loc}]"),)
			},
			LlccErrorKind::Context(error,) => {
				f.write_fmt(format_args!("{error:?}\nat: [{loc}]"),)
			},
			LlccErrorKind::Layer(error,) => {
				f.write_fmt(format_args!("{error:?}\nat: [{loc}]"),)
			},
			LlccErrorKind::Token(error,) => {
				f.write_fmt(format_args!("{error:?}\nat: [{loc}]"),)
			},
			LlccErrorKind::Unknown(error,) => {
				f.write_fmt(format_args!("{error:?}\nat: [{loc}]"),)
			},
		}
	}
}

impl std::error::Error for LlccError {}

impl From<io::Error,> for LlccError {
	#[track_caller]
	fn from(value: io::Error,) -> Self {
		Self { kind: LlccErrorKind::Io(value,), loc: Location::caller(), }
	}
}

impl From<std::num::ParseIntError,> for LlccError {
	#[track_caller]
	fn from(value: std::num::ParseIntError,) -> Self {
		Self {
			kind: LlccErrorKind::ParseSrcInt(value,),
			loc:  Location::caller(),
		}
	}
}

impl From<strum::ParseError,> for LlccError {
	#[track_caller]
	fn from(value: strum::ParseError,) -> Self {
		Self { kind: LlccErrorKind::Parse(value,), loc: Location::caller(), }
	}
}

impl From<&str,> for LlccError {
	#[track_caller]
	fn from(value: &str,) -> Self {
		Self {
			kind: LlccErrorKind::Unknown(value.to_string(),),
			loc:  Location::caller(),
		}
	}
}

impl From<TokenError,> for LlccError {
	#[track_caller]
	fn from(value: TokenError,) -> Self {
		Self { kind: LlccErrorKind::Token(value,), loc: Location::caller(), }
	}
}

impl From<LayerError,> for LlccError {
	#[track_caller]
	fn from(value: LayerError,) -> Self {
		Self { kind: LlccErrorKind::Layer(value,), loc: Location::caller(), }
	}
}

impl From<ImmTypeError,> for LlccError {
	#[track_caller]
	fn from(value: ImmTypeError,) -> Self {
		Self {
			kind: LlccErrorKind::ImmediateType(value,),
			loc:  Location::caller(),
		}
	}
}
