use crate::err::LlccB;

/// 副作用の有無を表す marker trait
/// 将来の制御用
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

pub trait Exec: Effect {
	type Status;
	fn exec(&self,) -> LlccB<Self::Status,>;
}
