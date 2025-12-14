use crate::parse::syntax::Lang;

pub mod arithmetic;

pub enum C {
	Arithmetic,
}

impl Lang for C {
	const NAME: &str = "c";
}
