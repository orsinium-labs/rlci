#[derive(Debug, PartialEq)]
pub enum Expression {
    Def {
        arg: String,
        expr: Box<Expression>,
    },
    Call {
        target: Box<Expression>,
        arg: Box<Expression>,
    },
    Assign {
        target: String,
        expr: Box<Expression>,
    },
    Id {
        name: String,
    },
}

impl Expression {
    pub fn make_def(t: (&str, Expression)) -> Expression {
        let (arg, expr) = t;
        Expression::Def {
            arg: arg.to_string(),
            expr: Box::new(expr),
        }
    }
    pub fn make_call(t: (Expression, Expression)) -> Expression {
        let (target, arg) = t;
        Expression::Call {
            target: Box::new(target),
            arg: Box::new(arg),
        }
    }
    pub fn make_assign(t: (&str, Expression)) -> Expression {
        let (target, expr) = t;
        Expression::Assign {
            target: target.to_string(),
            expr: Box::new(expr),
        }
    }
    pub fn make_id(name: &str) -> Expression {
        Expression::Id {
            name: name.to_string(),
        }
    }
}
