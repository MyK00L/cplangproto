use crate::ast::Ast;

pub fn run(a: Box<Ast>) {
	match *a {
		Ast::Id(x) => {},
		Ast::NumLiteral(x) => {},
		Ast::Access{lhs,rhs} => {},
		Ast::Call{lhs,args} => {},
		Ast::If{cond,then,els} => {},
		Ast::DoWhile{cond,then} => {},
		Ast::Break{amount} => {},
		Ast::Continue{amount} => {},
		Ast::Return{returnee} => {},
		Ast::Template{args,body} => {},
		Ast::Array{typ,size} => {},
		Ast::Pointer{typ} => {},
		Ast::TypDec{id,typ} => {},
		Ast::Var{typ,name} => {},
		Ast::Fun{name,args,typ,body} => {},
		Ast::Struct{name,els}=>{},
		Ast::Expr{body,ret}=>{},
	}
}
