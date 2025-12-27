#![feature(try_trait_v2)]
#![feature(error_generic_member_access)]
#![feature(tuple_trait)]
#![feature(generic_const_items)]
#![feature(associated_type_defaults)]

use crate::err::B::X;
// use crate::err::B::Y;
use crate::err::LlccB;
use crate::err::ReShape;
use std::path::PathBuf;

pub mod asm;
pub mod err;
pub mod front;
// pub mod parse;
pub mod orchestrator;
pub mod register;
pub mod semantics;

// trait ExpressionConverter {
// 	type Out: Evaluable;
// 	type In: Evaluable;
//
// 	fn convert<const REVERSIBLE: bool,>(self,);
// }
//
// trait Evaluable {
// 	type Rslt;
//
// 	fn eval(&self,) -> LlccB<Self::Rslt,>;
// }

fn stringify_path(path: impl Into<PathBuf,>,) -> LlccB<String,> {
	let p = path.into();
	let p = p.to_str().reshape("failed to get path",)?;
	X(p.to_string(),)
}

// #[cfg(test)] use std::fs;
// #[cfg(test)]
// fn path_check<F: Fn(T,) -> LlccB<R,>, T, R: Into<PathBuf,>,>(
// 	f: F,
// 	arg: T,
// ) -> LlccB<PathBuf,> {
// 	use colored::Colorize;
// 	use std::any::type_name;
//
// 	let p = f(arg,)?.into();
// 	assert!(
// 		fs::exists(&p)?,
// 		"returned path not exist at `{}`: {}",
// 		p.display(),
// 		type_name::<F,>().red().bold()
// 	);
// 	X(p,)
// }
