use crate::LlccB;
use crate::asm::run_cmd;
use llcc_error::B::X;
use llcc_error::ReShape;
use llcc_semantics::Ctx;
use llcc_semantics::purpose::CoreLayer;
use llcc_semantics::purpose::Layer;
use llcc_semantics::purpose::State;
use llcc_semantics::purpose::Worker;
use std::ops::Deref;
use std::ops::DerefMut;
use std::path::PathBuf;
use std::process::ExitStatus;

pub type Tokenizer =
	CoreLayer<TokenStream, TokenizerCtx, TokenizerWorker, Parser,>;

pub struct TokenStream(Vec<Token,>,);

impl TokenStream {
	pub fn new(inner: Vec<Token,>,) -> Self {
		Self(inner,)
	}
}

impl State for TokenStream {
	type Ctx = TokenizerCtx;

	fn update(&mut self, ctx: &Self::Ctx,) -> LlccB<(),> {
		self.0 = vec![];
		let mut src_chars = ctx.src.chars().peekable();

		while let Some(c,) = src_chars.next() {
			match c {
				a if a.is_whitespace() => {
					continue;
				},
				'+' => {
					self.0.push(Token::Sign(SignToken::Plus,),);
				},
				'-' => {
					self.0.push(Token::Sign(SignToken::Minus,),);
				},
				a if a.is_numeric() => {
					let mut s = String::from(a,);
					while src_chars.peek().is_some_and(|c| c.is_numeric(),) {
						s.push(src_chars.next().expect("must be some",),);
					}
					let n: i32 = s.parse()?;
					self.0.push(Token::Num(OpaqueSemanticToken::Determined(
						NumToken::Integer(n,),
					),),);
				},
				_ => todo!(),
			}
		}
		X((),)
	}
}

#[derive(Clone, Debug, PartialEq,)]
pub enum Token {
	Sign(SignToken,),
	Num(OpaqueSemanticToken<NumToken, String,>,),
	Ident(String,),
	Unknown(OpaqueSemanticToken<String, String,>,),
	End,
}

#[derive(Clone, Debug, PartialEq,)]
pub enum OpaqueSemanticToken<T, S,> {
	Determined(T,),
	UnClassified(S,),
}

#[derive(Clone, Debug, PartialEq,)]
pub enum SignToken {
	Plus,
	Minus,
}

#[derive(Clone, Debug, PartialEq,)]
pub enum NumToken {
	Integer(i32,),
	Decimal(f64,),
}

#[derive(Default,)]
pub struct TokenizerCtx {
	src: String,
}

impl TokenizerCtx {
	pub fn new(src: impl Into<String,>,) -> Self {
		Self { src: src.into(), }
	}
}

impl Ctx for TokenizerCtx {
	const ROLE: &'static str = "tokenizer ctx";
}

pub struct TokenizerWorker;

impl Worker<&TokenStream, TokenizerCtx,> for TokenizerWorker {
	type Output = Ast;

	fn work(
		&self,
		input: &TokenStream,
		ctx: &TokenizerCtx,
	) -> LlccB<Self::Output,> {
		let _ = input;
		let _ = ctx;
		todo!()
	}
}

pub type Parser = CoreLayer<Ast, ParserCtx, ParserWorker, SemanticCore,>;

pub struct Ast {
	root: Node,
}

impl State for Ast {
	type Ctx = ParserCtx;

	fn update(&mut self, ctx: &Self::Ctx,) -> LlccB<(),> {
		todo!()
	}
}

pub enum Node {
	Dummy(String,),
}

#[derive(Default,)]
pub struct ParserCtx;

impl Ctx for ParserCtx {
	const ROLE: &'static str = "parser ctx";
}

pub struct ParserWorker;

impl Worker<&Ast, ParserCtx,> for ParserWorker {
	type Output = SemanticCore;

	fn work(&self, input: &Ast, ctx: &ParserCtx,) -> LlccB<Self::Output,> {
		let _ = input;
		let _ = ctx;
		todo!()
	}
}

pub struct SemanticCore {}

impl Default for SemanticCore {
	fn default() -> Self {
		Self {}
	}
}

impl Layer for SemanticCore {
	type Ctx = Self;
	type Next = Self;
	type State = Self;
	type Worker = Self;

	fn from_state(state: Self::State,) -> Self {
		let _ = state;
		todo!()
	}

	fn state(&self,) -> &Self::State {
		todo!()
	}

	fn state_mut(&mut self,) -> &mut Self::State {
		todo!()
	}

	fn ctx(&self,) -> &Self::Ctx {
		todo!()
	}

	fn ctx_mut(&mut self,) -> &mut Self::Ctx {
		todo!()
	}

	fn set_worker(&mut self, worker: Self::Worker,) -> &mut Self {
		let _ = worker;
		todo!()
	}

	fn next(&self,) -> LlccB<Self::Next,> {
		todo!()
	}
}

impl State for SemanticCore {
	type Ctx = ();

	fn update(&mut self, ctx: &Self::Ctx,) -> LlccB<(),> {
		todo!()
	}
}

impl Ctx for SemanticCore {
	const ROLE: &'static str = "semantic core";
}

impl<I, Ctx,> Worker<I, Ctx,> for SemanticCore {
	type Output = Self;

	fn work(&self, _input: I, _ctx: &Ctx,) -> LlccB<Self::Output,> {
		unimplemented!()
	}
}

pub fn exec(exe_path: impl Into<PathBuf,>,) -> LlccB<ExitStatus,> {
	run_cmd::<[&str; 0], &str,>(
		exe_path.into().to_str().reshape("failed to stringify exe_path",)?,
		[],
	)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_tokenstream_update() -> LlccB<(),> {
		let mut tokenstream = TokenStream::new(vec![],);
		let exprs = ["1+1", "1-1", "1 + 1", "1 + 2 - 3+5 +4 -5+7",];

		let tokens_list: Vec<_,> = exprs
			.iter()
			.map(|s| {
				tokenstream.update(&TokenizerCtx::new(*s,),)?;
				let inner = tokenstream.0.clone();
				X(inner,)
			},)
			.try_collect()?;

		assert_eq!(
			tokens_list[0],
			vec![
				Token::Num(OpaqueSemanticToken::Determined(NumToken::Integer(
					1
				))),
				Token::Sign(SignToken::Plus),
				Token::Num(OpaqueSemanticToken::Determined(NumToken::Integer(
					1
				))),
			],
		);
		assert_eq!(
			tokens_list[1],
			vec![
				Token::Num(OpaqueSemanticToken::Determined(NumToken::Integer(
					1
				))),
				Token::Sign(SignToken::Minus),
				Token::Num(OpaqueSemanticToken::Determined(NumToken::Integer(
					1
				))),
			],
		);
		assert_eq!(
			tokens_list[2],
			vec![
				Token::Num(OpaqueSemanticToken::Determined(NumToken::Integer(
					1
				))),
				Token::Sign(SignToken::Plus),
				Token::Num(OpaqueSemanticToken::Determined(NumToken::Integer(
					1
				))),
			],
		);
		assert_eq!(
			tokens_list[3],
			vec![
				Token::Num(OpaqueSemanticToken::Determined(NumToken::Integer(
					1
				))),
				Token::Sign(SignToken::Plus),
				Token::Num(OpaqueSemanticToken::Determined(NumToken::Integer(
					2
				))),
				Token::Sign(SignToken::Minus),
				Token::Num(OpaqueSemanticToken::Determined(NumToken::Integer(
					3
				))),
				Token::Sign(SignToken::Plus),
				Token::Num(OpaqueSemanticToken::Determined(NumToken::Integer(
					5
				))),
				Token::Sign(SignToken::Plus),
				Token::Num(OpaqueSemanticToken::Determined(NumToken::Integer(
					4
				))),
				Token::Sign(SignToken::Minus),
				Token::Num(OpaqueSemanticToken::Determined(NumToken::Integer(
					5
				))),
				Token::Sign(SignToken::Plus),
				Token::Num(OpaqueSemanticToken::Determined(NumToken::Integer(
					7
				))),
			],
		);
		X((),)
	}
}
