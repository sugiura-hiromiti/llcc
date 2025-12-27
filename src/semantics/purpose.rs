use crate::semantics::Convert;
use crate::semantics::ability::ReadIn;
use crate::semantics::c::HasOut;
use crate::semantics::context::HasIn;

pub trait Layer: Convert + HasIn + HasOut {}

pub trait LayerBuilder: HasIn + HasOut + ReadIn {}
