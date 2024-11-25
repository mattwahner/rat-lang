use crate::grammar::parser::{Binary, Expression, Integer, TermOperator};

trait Evaluate<T> {
    fn evaluate(&self) -> T;
}

impl Evaluate<i32> for Expression {
    fn evaluate(&self) -> i32 {
        match self {
            Expression::Binary(b) => b.evaluate(),
            Expression::Integer(i) => i.evaluate(),
        }
    }
}

impl Evaluate<i32> for Binary {
    fn evaluate(&self) -> i32 {
        let left = self.left.evaluate();
        let right = self.right.evaluate();
        match self.operator {
            TermOperator::Plus => left + right,
            TermOperator::Minus => left - right,
        }
    }
}

impl Evaluate<i32> for Integer {
    fn evaluate(&self) -> i32 {
        self.value
    }
}

pub fn evaluate(root: &Expression) -> i32 {
    root.evaluate()
}
