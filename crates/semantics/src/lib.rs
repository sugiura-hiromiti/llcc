#![feature(try_trait_v2)]
#![feature(unboxed_closures)]
#![feature(tuple_trait)]
#![feature(fn_traits)]
#![feature(associated_type_defaults)]
#![feature(impl_trait_in_assoc_type)]

use llcc_error::LlccError;
use std::any::type_name;
use std::panic::Location;

pub mod ability;
pub mod context;
pub mod purpose;

/// 表現変換
// pub trait Convert<LayerFrom, LayerTo: p::Layer,> {
// 	fn convert(&self,);
// }

/// Ctxは与えられる物限定
/// それ以外は状態と見るべき
pub trait Ctx: Default {
	/// description of context role
	const ROLE: &'static str;
}

pub trait CtxErr {
	fn lack_of_ctx<C: Ctx,>() -> Self;
}

impl CtxErr for LlccError {
	#[track_caller]
	fn lack_of_ctx<C: Ctx,>() -> Self {
		LlccError::LackOfContext {
			context_role: C::ROLE,
			type_name:    type_name::<C,>(),
			loc:          Location::caller(),
		}
	}
}
