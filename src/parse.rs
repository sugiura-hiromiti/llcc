//! source code → token stream → Ast

use crate::err::LlccB;

pub mod syntax;

pub struct Parser<'a, T,> {
	pos: usize,
	src: &'a [T],
}

impl<'a,> Parser<'a, char,> {
	pub fn parse(self,) {}
}

/// `Ast`型は文法構造の定義をする
pub struct Ast {}

/// `SyntaxTree`型は実際のプログラムの構造を格納する
/// ドメイン的には、文法を木構造として表現する
pub struct SyntaxTree {}

// pub struct Node<'a,> {
// 	dep: &'a [Node<'a,>],
// 	val: Value,
// }
//
// pub enum Value {
// 	/// literal
// 	Lit(LitKind,),
// 	/// name
// 	Ident { name: String, },
// }
//
// pub enum LitKind {
// 	Num(i32,),
// }
//
// struct ValueBuilder {
// 	token_start: usize,
// 	token_end:   usize,
// }
//
//
// impl<'a,> Parser<'a, char,> {
// 	pub fn parse<'b,>(mut self,) -> Ast<'b,> {
// 		let len = self.src.len();
//
// 		while self.pos < len {
// 			let c = &self.src[self.pos];
//
// 			match c {
// 				n if n.is_digit(10)=>
// 			}
// 		}
// 		todo!()
// 	}
//
// fn apply_syntax<S:Syntax>(&mut self)-> S{
//
// }
// }
