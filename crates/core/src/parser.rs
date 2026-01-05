use crate::assume;
use crate::expect;
use crate::tokenizer::NumToken;
use crate::tokenizer::SignToken;
use crate::tokenizer::Token;
use crate::tokenizer::TokenStream;
use crate::tokenizer::Tokenizer;
use crate::tokenizer::TokenizerCtx;
use llcc_error::B::X;
use llcc_error::B::Y;
use llcc_error::LlccB;
use llcc_error::LlccError;
use llcc_error::TokenError;
use llcc_semantics::Ctx;
use llcc_semantics::purpose::CoreLayer;
use llcc_semantics::purpose::Lowering;
use llcc_semantics::purpose::State;
use llcc_semantics::purpose::Worker;
use std::marker::PhantomData;

pub type Parser<O,> =
	CoreLayer<Ast, ParserCtx, fn(&Ast, &ParserCtx,) -> LlccB<O,>,>;

pub struct Ast {
	root: Box<Node,>,
}

impl Ast {
	pub fn new(root: Box<Node,>,) -> Self {
		Self { root, }
	}
}

impl State for Ast {}

pub struct Node {
	val: Value,
	l:   Option<Box<Node,>,>,
	r:   Option<Box<Node,>,>,
}

impl Node {
	fn node(
		val: Value,
		l: Option<Box<Node,>,>,
		r: Option<Box<Node,>,>,
	) -> Self {
		Self { val, l, r, }
	}
}

fn expr(ts: &TokenStream, ctx: &mut TokenizerCtx,) -> LlccB<Box<Node,>,> {
	let mut node = mul(ts, ctx,)?;

	loop {
		let val = if assume!(&Token::Sign(SignToken::Plus,), ts, ctx,) {
			Value::Op(OpValue::Plus,)
		} else if assume!(&Token::Sign(SignToken::Minus,), ts, ctx,) {
			Value::Op(OpValue::Minus,)
		} else {
			return X(node,);
		};

		node = Box::new(Node::node(val, Some(node,), Some(mul(ts, ctx,)?,),),);
	}
}

fn mul(ts: &TokenStream, ctx: &mut TokenizerCtx,) -> LlccB<Box<Node,>,> {
	let mut node = primary(ts, ctx,)?;

	loop {
		let val = if assume!(&Token::Sign(SignToken::Mul,), ts, ctx,) {
			Value::Op(OpValue::Mul,)
		} else if assume!(&Token::Sign(SignToken::Div,), ts, ctx,) {
			Value::Op(OpValue::Div,)
		} else {
			return X(node,);
		};

		node =
			Box::new(Node::node(val, Some(node,), Some(primary(ts, ctx,)?,),),);
	}
}

fn primary(ts: &TokenStream, ctx: &mut TokenizerCtx,) -> LlccB<Box<Node,>,> {
	if assume!(&Token::Sign(SignToken::LeftParen,), ts, ctx,) {
		let node = expr(ts, ctx,)?;
		expect!(&Token::Sign(SignToken::RightParen,), ts, ctx,)?;
		return X(node,);
	}
	todo!()
}

pub enum Value {
	Op(OpValue,),
	Literal(LiteralValue,),
}

impl Value {
	fn from_token(token: &Token,) -> LlccB<Self,> {
		match token {
			Token::Sign(sign_token,) => Self::from_sign_token(sign_token,),
			Token::Num(opaque_semantic_token,) => match opaque_semantic_token {
				crate::tokenizer::OpaqueSemanticToken::Determined(
					num_token,
				) => X(Self::from_num_token(num_token,),),
				crate::tokenizer::OpaqueSemanticToken::UnClassified(_s,) => {
					todo!()
				},
			},
			Token::Ident(_,) => todo!(),
			Token::Unknown(_opaque_semantic_token,) => todo!(),
			Token::End => Y(LlccError::from(TokenError::UnexpectedToken,),),
		}
	}

	fn from_sign_token(sign_token: &SignToken,) -> LlccB<Self,> {
		match sign_token {
			SignToken::Plus => X(Self::Op(OpValue::Plus,),),
			SignToken::Minus => X(Self::Op(OpValue::Minus,),),
			SignToken::Mul => X(Self::Op(OpValue::Mul,),),
			SignToken::Div => X(Self::Op(OpValue::Div,),),
			SignToken::LeftParen => {
				Y(LlccError::from(TokenError::UnexpectedToken,),)
			},
			SignToken::RightParen => {
				Y(LlccError::from(TokenError::UnexpectedToken,),)
			},
		}
	}

	fn from_num_token(num_token: &NumToken,) -> Self {
		match num_token {
			NumToken::Integer(i,) => Self::Literal(LiteralValue::Integer(*i,),),
			NumToken::Decimal(f,) => Self::Literal(LiteralValue::Decimal(*f,),),
		}
	}
}

pub enum OpValue {
	Plus,
	Minus,
	Mul,
	Div,
}

pub enum LiteralValue {
	Integer(i32,),
	Decimal(f64,),
}

#[derive(Default,)]
pub struct ParserCtx;

impl Ctx for ParserCtx {
	const ROLE: &'static str = "parser ctx";
}

pub struct FromTokenizer<O,>(PhantomData<O,>,);

impl<O,> Lowering for FromTokenizer<O,> {
	type Upper = Tokenizer<Parser<O,>,>;
}

impl<'a, O,> Worker<&'a TokenStream, TokenizerCtx,> for FromTokenizer<O,> {
	type Output = Parser<O,>;

	fn work(
		&self,
		input: &'a TokenStream,
		ctx: &TokenizerCtx,
	) -> LlccB<Self::Output,> {
		let idx = ctx.idx();
		let mut ctx = TokenizerCtx::new();
		*ctx.idx_mut() = idx;
		let root = expr(input, &mut ctx,)?;
		let parser = Parser::new(Ast::new(root,), ParserCtx,);
		X(parser,)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::tokenizer::OpaqueSemanticToken;

	#[test]
	fn test_value_from_sign_token_ops() -> LlccB<(),> {
		let plus = Value::from_sign_token(&SignToken::Plus,)?;
		let minus = Value::from_sign_token(&SignToken::Minus,)?;
		let mul = Value::from_sign_token(&SignToken::Mul,)?;
		let div = Value::from_sign_token(&SignToken::Div,)?;

		assert!(matches!(plus, Value::Op(OpValue::Plus)));
		assert!(matches!(minus, Value::Op(OpValue::Minus)));
		assert!(matches!(mul, Value::Op(OpValue::Mul)));
		assert!(matches!(div, Value::Op(OpValue::Div)));
		X((),)
	}

	#[test]
	fn test_value_from_sign_token_paren_error() -> LlccB<(),> {
		let left = Value::from_sign_token(&SignToken::LeftParen,);
		let right = Value::from_sign_token(&SignToken::RightParen,);

		assert!(matches!(left, Y(_)));
		assert!(matches!(right, Y(_)));

		if let Y(err,) = left {
			let msg = format!("{err}");
			assert!(msg.contains("UnexpectedToken"));
		}
		if let Y(err,) = right {
			let msg = format!("{err}");
			assert!(msg.contains("UnexpectedToken"));
		}

		X((),)
	}

	#[test]
	fn test_value_from_token_number() -> LlccB<(),> {
		let int = Token::Num(OpaqueSemanticToken::Determined(
			NumToken::Integer(42,),
		),);
		let dec = Token::Num(OpaqueSemanticToken::Determined(
			NumToken::Decimal(3.5,),
		),);

		let int_value = Value::from_token(&int,)?;
		let dec_value = Value::from_token(&dec,)?;

		assert!(matches!(
			int_value,
			Value::Literal(LiteralValue::Integer(42))
		));
		assert!(matches!(
			dec_value,
			Value::Literal(LiteralValue::Decimal(3.5))
		));
		X((),)
	}

	#[test]
	fn test_value_from_token_end_error() -> LlccB<(),> {
		let end = Token::End;
		let err = Value::from_token(&end,);
		assert!(matches!(err, Y(_)));
		if let Y(err,) = err {
			let msg = format!("{err}");
			assert!(msg.contains("UnexpectedToken"));
		}
		X((),)
	}
}
