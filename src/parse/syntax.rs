use std::marker::PhantomData;

use crate::parse::SyntaxTree;

pub mod c;

// pub trait Syntax<L: Lang,> {
// 	fn to_tree(self,) -> SyntaxTree;
// 	// fn from_tree(stream: &[ValueBuilder],) -> LlccB<Self,>;
// }
//
// pub trait Lang {
// 	const NAME: &str;
// }
//
// struct Token<const VALUE: char,>;
//
// impl<const V: char,> Syntax<Void,> for Token<V,> {
// 	fn to_tree(self,) -> SyntaxTree {
// 		todo!()
// 	}
// }
//
// struct Alpha<'a,> {
// 	body: &'a str,
// }
//
// struct Num {
// 	body: i32,
// }
//
// struct Void;
//
// impl Syntax<Void,> for Void {
// 	fn to_tree(self,) -> SyntaxTree {
// 		todo!()
// 	}
// }
//
// impl Lang for Void {
// 	const NAME: &str = "parser";
// }
//
// struct Repeat<S: Syntax<L,>, L: Lang,> {
// 	body: S,
// 	_b:   PhantomData<L,>,
// }
//
// struct And<A: Syntax<L,>, B: Syntax<L,>, L: Lang,> {
// 	a: A,
// 	b: B,
// }
//
// impl<A: Syntax<L,>, B: Syntax<L,>, L: Lang,> Syntax<L,> for And<A, B, L,> {
// 	fn to_tree(self,) -> SyntaxTree {
// 		todo!()
// 	}
// }
//
// enum Or<A: Syntax<L,>, B: Syntax<L,>, L: Lang,> {
// 	A(A,),
// 	B(B,),
// }
//
// impl<A: Syntax<L,>, B: Syntax<L,>, L: Lang,> Syntax<L,> for Or<A, B, L,> {
// 	fn to_tree(self,) -> SyntaxTree {
// 		todo!()
// 	}
// }
//
// struct Opt<S: Syntax<L,>, L: Lang,> {
// 	body: Or<S, Void, L,>,
// }
//
// // struct Opt<S,> {
// // 	body: PhantomData<S,>,
// // }
// //
// // struct OneOf<S: Tuple,> {
// // 	body: PhantomData<S,>,
// // }
// //
// // struct Combine<S: Tuple,> {
// // 	//	body: PhantomData<S,>,
// // 	a: S,
// // }
//
// // impl<S: Tuple,> Combine<S,> {
// // 	fn a(self,) {}
// // }
//
// enum AtomicSyntax<'a,> {
// 	Alpha,
// 	Num,
// 	Token(&'a str,),
// }
