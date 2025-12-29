use llcc_error::LlccB;

/// 副作用の有無を表す marker trait
/// 将来の制御用
// pub trait Effect<const PURE: bool,> {}
pub trait Effect {}

// backendがfsとは限らない
pub trait WriteOut: Effect {
	type Out;
	fn emit(&self,) -> LlccB<Self::Out,>;
}

pub trait ReadIn {
	type In;
	fn load(&self,) -> LlccB<Self::In,>;
}

/// `Convert`とは別物
pub trait Eval: Effect {
	type Status;
	fn eval(&self,) -> LlccB<Self::Status,>;
}
