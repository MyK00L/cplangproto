#[derive(Clone, Debug)]
pub enum Ast {
	Id(String),
	NumLiteral(String),
	StringLiteral(String),

	// operations
	Access {
		lhs: Box<Ast>,
		rhs: Box<Ast>,
	},
	Call {
		lhs: Box<Ast>,
		args: Vec<Box<Ast>>,
	},

	// flow control
	If {
		cond: Box<Ast>,
		then: Box<Ast>,
		els: Option<Box<Ast>>,
	},
	DoWhile {
		cond: Box<Ast>,
		then: Box<Ast>,
	},
	Break {
		amount: Box<Ast>,
	},
	Continue {
		amount: Box<Ast>,
	},
	Return {
		returnee: Box<Ast>,
	},

	// template
	Template {
		args: Vec<Box<Ast>>,
		body: Box<Ast>,
	},

	// types
	Array {
		typ: Box<Ast>,
		size: Box<Ast>,
	},
	Pointer {
		typ: Box<Ast>,
	},
	Reference {
		typ: Box<Ast>,
	},

	// name declarations
	Var {
		typ: Box<Ast>,
		name: Box<Ast>,
	},
	Fun {
		name: Box<Ast>,
		args: Vec<(Box<Ast>,Box<Ast>)>,
		typ: Box<Ast>,
		body: Box<Ast>,
	},
	Struct {
		name: Box<Ast>,
		els: Vec<(Box<Ast>,Option<Box<Ast>>)>,
	},

	// everything
	Expr {
		body: Vec<Box<Ast>>,
		ret: Option<Box<Ast>>,
	},
}

