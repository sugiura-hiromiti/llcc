//! <expr>   ::= <term> { ("+" | "-") <term> }
//!
//! <term>   ::= <factor> { ("*" | "/") <factor> }
//!
//! <factor> ::= "(" <expr> ")"
//!            | <number>

// use crate::parse::syntax::And;
// use crate::parse::syntax::Num;
// use crate::parse::syntax::Or;
// use crate::parse::syntax::Repeat;
// use crate::parse::syntax::Syntax;
// use crate::parse::syntax::Token;
// use crate::parse::syntax::c::C;
//
// pub struct Expr<'a,> {
// 	term:   Term<'a,>,
// 	repeat: Repeat<And<Or<Token<'+',>, Token<'-',>,>, Term<'a,>,>,>,
// }
//
// impl<'a,> Syntax<C,> for Expr<'a,> {
// 	fn to_tree(self,) -> crate::parse::SyntaxTree {
// 		todo!()
// 	}
// }
//
// pub struct Term<'a,> {
// 	factor: Factor<'a,>,
// 	repeat: Repeat<And<Or<Token<'*',>, Token<'/',>,>, Factor<'a,>,>,>,
// }
//
// impl<'a,> Syntax<C,> for Term<'a,> {
// 	fn to_tree(self,) -> crate::parse::SyntaxTree {
// 		todo!()
// 	}
// }
//
// pub enum Factor<'a,> {
// 	Expr { pre: Token<'(',>, expr: &'a Expr<'a,>, post: Token<')',>, },
// 	Number(Num,),
// }
//
// impl<'a,> Syntax<C,> for Factor<'a,> {
// 	fn to_tree(self,) -> crate::parse::SyntaxTree {
// 		todo!()
// 	}
// }
