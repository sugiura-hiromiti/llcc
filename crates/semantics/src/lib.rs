#![feature(try_trait_v2)]

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

pub trait Ctx {
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
