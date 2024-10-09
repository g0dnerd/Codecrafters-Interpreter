use crate::expression::Expression;

use crate::token::{ Token, LiteralValue };

pub struct Binary {
	left: Box<dyn Expression>,
	operator: Token,
	right: Box<dyn Expression>,
}

pub struct Grouping {
	expression: Box<dyn Expression>,
}

pub struct Literal {
	value: Box<dyn LiteralValue>,
}

pub struct Unary {
	operator: Token,
	right: Box<dyn Expression>,
}

