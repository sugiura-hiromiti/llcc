use crate::front::SrcRef;
use crate::parser::Ast;
use llcc_error::B::X;
use llcc_error::B::Y;
use llcc_error::LlccB;
use llcc_error::LlccError;
use llcc_error::TokenError;
use llcc_semantics::Ctx;
use llcc_semantics::purpose::CoreLayer;
use llcc_semantics::purpose::State;
use llcc_semantics::purpose::Worker;
use std::ops::Deref;

// pub type Tokenizer = CoreLayer<TokenStream, TokenizerCtx, TokenizerWorker,>;
pub type Tokenizer<O,> = CoreLayer<
	TokenStream,
	TokenizerCtx,
	fn(&TokenStream, &TokenizerCtx,) -> LlccB<O,>,
>;

#[derive(Debug,)]
pub struct TokenStream(Vec<Token,>,);

impl Deref for TokenStream {
	type Target = Vec<Token,>;

	fn deref(&self,) -> &Self::Target {
		&self.0
	}
}

impl PartialEq<Vec<Token,>,> for TokenStream {
	fn eq(&self, other: &Vec<Token,>,) -> bool {
		&self.0 == other
	}
}

impl TokenStream {
	pub fn new(inner: Vec<Token,>,) -> Self {
		Self(inner,)
	}

	//  NOTE: 分割の責務に集中する
	//  Tokenの生成はTokenにまかせる
	pub fn into_stream(src: impl SrcRef,) -> LlccB<Self,> {
		let src = src.source_code()?;
		let mut src_chars = src.chars().peekable();

		let mut tokens = vec![];
		while let Some(c,) = src_chars.next() {
			match c {
				a if a.is_whitespace() => {
					continue;
				},
				'+' | '-' | '*' | '/' | '(' | ')' => {
					tokens.push(Token::Sign(SignToken::from_char(c,)?,),);
				},
				a if a.is_numeric() => {
					let mut s = String::from(a,);
					while src_chars.peek().is_some_and(|c| c.is_numeric(),) {
						s.push(src_chars.next().expect("must be some",),);
					}
					let n: i32 = s.parse()?;
					tokens.push(Token::Num(OpaqueSemanticToken::Determined(
						NumToken::Integer(n,),
					),),);
				},
				_ => todo!(),
			}
		}

		X(Self(tokens,),)
	}
}

impl State for TokenStream {}

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
	Mul,
	Div,
	LeftParen,
	RightParen,
}

impl SignToken {
	fn from_char(c: char,) -> LlccB<Self,> {
		let token = match c {
			'+' => Self::Plus,
			'-' => Self::Minus,
			'*' => Self::Mul,
			'/' => Self::Div,
			'(' => Self::LeftParen,
			')' => Self::RightParen,
			_ => return Y(LlccError::from(TokenError::UnexpectedToken,),),
		};
		X(token,)
	}
}

#[derive(Clone, Debug, PartialEq,)]
pub enum NumToken {
	Integer(i32,),
	Decimal(f64,),
}

#[derive(Default,)]
pub struct TokenizerCtx {
	idx: usize,
}

impl TokenizerCtx {
	pub fn new() -> Self {
		Self { idx: 0, }
	}

	pub fn idx(&self,) -> usize {
		self.idx
	}

	pub fn idx_mut(&mut self,) -> &mut usize {
		&mut self.idx
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

// pub fn assume(
// 	token: &Token, ts: &TokenStream, ctx: &mut TokenizerCtx,
// ) -> bool {
// 	let idx = ctx.idx();
// 	match ts.get(idx,) {
// 		Some(t,) if t == token => {
// 			*ctx.idx_mut() = idx + 1;
// 			true
// 		},
// 		_ => false,
// 	}
// }

// pub fn expect(
// 	token: &Token,
// 	ts: &TokenStream,
// 	ctx: &mut TokenizerCtx,
// ) -> LlccB<(),> {
// 	// if assume(token, ts, ctx,) {
// 	// 	X((),)
// 	// } else {
// 	// 	Y(LlccError::from(TokenError::UnexpectedToken,),)
// 	// }
// 	let idx = ctx.idx();
// 	match ts.get(idx) {
// 	    Some(t) if t==token =>
// 	}
// }

#[macro_export]
macro_rules! expect {
	($token:pat, $ts:expr, $ctx:expr,) => {{
		let idx = $ctx.idx();
		match $ts.get(idx,) {
			a @ Some($token,) => {
				*$ctx.idx_mut() = idx + 1;
				X(a,)
			},
			_a => Y(LlccError::from(TokenError::UnexpectedToken,),),
		}
	}};
}

#[macro_export]
macro_rules! assume {
	($token:pat, $ts:expr, $ctx:expr,) => {
		matches!(expect!($token, $ts, $ctx,), X(_))
	};
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_tokenstream_update() -> LlccB<(),> {
		let exprs = [
			"1+1",
			"1-1",
			"1 + 1",
			"1 + 2 - 3+5 +4 -5+7",
			"98 / 7 - 6",
		];

		let tokens_list: Vec<_,> = exprs
			.iter()
			.map(|s| TokenStream::into_stream(s.to_string(),),)
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
		assert_eq!(
			tokens_list[4],
			vec![
				Token::Num(OpaqueSemanticToken::Determined(NumToken::Integer(
					98
				))),
				Token::Sign(SignToken::Div),
				Token::Num(OpaqueSemanticToken::Determined(NumToken::Integer(
					7
				))),
				Token::Sign(SignToken::Minus),
				Token::Num(OpaqueSemanticToken::Determined(NumToken::Integer(
					6
				))),
			],
		);
		X((),)
	}

	#[test]
	fn test_tokenstream_parens_and_mul_div() -> LlccB<(),> {
		let ts = TokenStream::into_stream("(12+3)*4/2",)?;
		assert_eq!(
			ts,
			vec![
				Token::Sign(SignToken::LeftParen),
				Token::Num(OpaqueSemanticToken::Determined(NumToken::Integer(
					12
				))),
				Token::Sign(SignToken::Plus),
				Token::Num(OpaqueSemanticToken::Determined(NumToken::Integer(
					3
				))),
				Token::Sign(SignToken::RightParen),
				Token::Sign(SignToken::Mul),
				Token::Num(OpaqueSemanticToken::Determined(NumToken::Integer(
					4
				))),
				Token::Sign(SignToken::Div),
				Token::Num(OpaqueSemanticToken::Determined(NumToken::Integer(
					2
				))),
			],
		);
		X((),)
	}

	#[test]
	fn test_tokenizer_new() -> LlccB<(),> {
		let ts = TokenStream::into_stream("1+1",)?;
		let ctx = TokenizerCtx::new();
		let _tokenizer = Tokenizer::<(),>::new(ts, ctx,);
		X((),)
	}
}
