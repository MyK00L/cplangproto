#[derive(Clone, Debug)]
pub enum Ast {
	Id(String),
	NumLiteral(String),

	Index {
		lhs: Box<Ast>,
		rhs: Box<Ast>,
	},
	Access {
		lhs: Box<Ast>,
		rhs: Box<Ast>,
	},
	Call {
		lhs: Box<Ast>,
		args: Vec<Box<Ast>>,
	},
	RightPlusPlus {
		lhs: Box<Ast>,
	},
	RightMinusMinus {
		lhs: Box<Ast>,
	},

	Lnot {
		rhs: Box<Ast>,
	},
	Bnot {
		rhs: Box<Ast>,
	},
	Neg {
		rhs: Box<Ast>,
	},
	Deref {
		rhs: Box<Ast>,
	},
	Ref {
		rhs: Box<Ast>,
	},

	Mul {
		lhs: Box<Ast>,
		rhs: Box<Ast>,
	},
	Div {
		lhs: Box<Ast>,
		rhs: Box<Ast>,
	},
	Mod {
		lhs: Box<Ast>,
		rhs: Box<Ast>,
	},

	Add {
		lhs: Box<Ast>,
		rhs: Box<Ast>,
	},
	Sub {
		lhs: Box<Ast>,
		rhs: Box<Ast>,
	},

	Rhs {
		lhs: Box<Ast>,
		rhs: Box<Ast>,
	},
	Lhs {
		lhs: Box<Ast>,
		rhs: Box<Ast>,
	},

	Less {
		lhs: Box<Ast>,
		rhs: Box<Ast>,
	},
	LessEqual {
		lhs: Box<Ast>,
		rhs: Box<Ast>,
	},

	Equal {
		lhs: Box<Ast>,
		rhs: Box<Ast>,
	},
	Nequal {
		lhs: Box<Ast>,
		rhs: Box<Ast>,
	},

	Band {
		lhs: Box<Ast>,
		rhs: Box<Ast>,
	},

	Bxor {
		lhs: Box<Ast>,
		rhs: Box<Ast>,
	},

	Bor {
		lhs: Box<Ast>,
		rhs: Box<Ast>,
	},

	Land {
		lhs: Box<Ast>,
		rhs: Box<Ast>,
	},

	Lor {
		lhs: Box<Ast>,
		rhs: Box<Ast>,
	},

	Assign {
		lhs: Box<Ast>,
		rhs: Box<Ast>,
	},

	Vde {
		lhs: Box<Ast>,
		rhs: Option<Box<Ast>>,
	},
	VariableDeclaration {
		typ: Box<Ast>,
		vars: Vec<Box<Ast>>,
	},

	If {
		cond: Box<Ast>,
		then: Box<Ast>,
		els: Option<Box<Ast>>,
	},
	While {
		cond: Box<Ast>,
		then: Box<Ast>,
	},
	Break {
		amount: u32,
	},
	Continue {
		amount: u32,
	},
	Return {
		returnee: Box<Ast>,
	},

	StructDefEl {
		typ: Box<Ast>,
		name: Option<Box<Ast>>,
	},
	StructDef {
		name: Box<Ast>,
		els: Vec<Box<Ast>>,
	},
	
	Array {
		typ: Box<Ast>,
		size: Box<Ast>,
	},
	Pointer {
		typ: Box<Ast>,
	},
	TypDec {
		id: Box<Ast>,
		typ: Box<Ast>,
	},
	
	Template {
		args: Vec<Box<Ast>>,
		body: Box<Ast>,
	},

	FunArgsEl {
		typ: Box<Ast>,
		name: Box<Ast>,
	},
	Fun {
		name: Box<Ast>,
		args: Vec<Box<Ast>>,
		typ: Box<Ast>,
		body: Box<Ast>,
	},
	Expr {
		body: Vec<Box<Ast>>,
		ret: Option<Box<Ast>>,
	},
}
/*

=
+=   -=
*=   /=   %=
<<=   >>=
&=   ^=   |=
#[derive(Clone,Debug)]
pub enum Token{
	If, // if
	Else, // else
	While, // while
	For, // for
	Loop, // loop
	Break, // break
	Continue, // continue
	Return, // return
	Fun, // fn
	Struct, // struct
	Type, // type
	Alias, // alias

	Lparen, // (
	Rparen, // )
	Lbracket, // [
	Rbracket, // ]
	Lbrace, // {
	Rbrace, // }
	SemiColon, // ;
	Comma, // ,
	Dot, // .
	Colon, // :
	Equal, // =
	EqualEqual, // ==
	Plus, // +
	PlusEqual, // +=
	PlusPlus, // ++
	Minus, // -
	MinusEqual, // -=
	MinusMinus, // --
	Star, // *
	StarEqual, // *=
	Slash, // /
	SlashEqual, // /=
	Mod, // %
	ModEqual, // %=
	Less, // <
	LessEqual, // <=
	LessLess, // <<
	Greater, // >
	GreaterEqual, // >=
	GreaterGreater, // >>
	Exclamation, // !
	NotEqual, // !=
	Question, // ?
	And, // &
	AndEqual, // &=
	AndAnd, // &&
	Or, // |
	OrEqual, // |=
	OrOr, // ||
	Xor, // ^
	XorEqual, // ^=
	Tilde, // ~
}
*/
