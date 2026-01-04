use crate::semantic_core::SemanticCore;
use crate::tokenizer::Tokenizer;
use llcc_error::LlccB;
use llcc_semantics::Ctx;
use llcc_semantics::purpose::CoreLayer;
use llcc_semantics::purpose::Lowering;
use llcc_semantics::purpose::State;
use llcc_semantics::purpose::Worker;

pub type Parser<O,> =
	CoreLayer<Ast, ParserCtx, fn(&Ast, &ParserCtx,) -> LlccB<O,>,>;

pub struct Ast {
	root: Node,
}

impl State for Ast {}

pub struct Node {
	val: Value,
	l:   Option<Box<Node,>,>,
	r:   Option<Box<Node,>,>,
}

pub enum Value {
	Op(OpValue,),
	Literal(LiteralValue,),
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

pub struct FromTokenizer;

impl Lowering for FromTokenizer {
	type Upper = Tokenizer<impl Worker,>;
}
