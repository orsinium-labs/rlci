use super::GlobalScope;
use crate::ast_nodes::Expr;
use anyhow::Context;

/// Different types of runtime values.
///
/// Runtime values map quite closely to AST ("homoiconicity"),
/// and `from_expr` and `repr` methods connect AST and Value together.
///
/// In a bigger language, perhaps it's a good idea to use traits and let each
/// type to be defined separately. However, for small languages (up to JSON)
/// enums work quite well.
#[derive(Debug, Clone)]
pub enum Value {
    /// A lambda function definition
    Def {
        arg: String,
        value: Box<Value>,
    },
    /// An unbound identifier.
    Id {
        name: String,
    },
    /// An identifier bound to a global or local Value.
    BoundId {
        name: String,
        value: Box<Value>,
        global: bool,
    },
    // A function application.
    Call {
        target: Box<Value>,
        arg: Box<Value>,
    },
}

impl Value {
    /// Convert an AST node into a Value.
    ///
    /// `Expr` and `Value` are very similar but not the same.
    /// Their semantic is slightly different and then `Value` may contain
    /// runtime-only infromation. In our case, IDs can be bound,
    /// so `Value` additionally has `BoundId`.
    pub fn from_expr(expr: &Expr) -> Self {
        match expr {
            Expr::Def { arg, expr } => Value::Def {
                arg: arg.to_string(),
                // The `into` method is a convenient way to convert `Value`
                // into `Box<Value>`.
                value: Value::from_expr(expr).into(),
            },
            Expr::Call { target, arg } => Value::Call {
                target: Value::from_expr(target).into(),
                arg: Value::from_expr(arg).into(),
            },
            Expr::Id { name } => Value::Id {
                name: name.to_string(),
            },
        }
    }

    /// Represent the value as a valid human-readable expression.
    ///
    /// The name and the idea comes from Python's `__repr__` magic method.
    /// The main idea is that you can copy-paste the result of this function into REPL
    /// and (assuming all globals are the same) get the same evaluation result.
    ///
    /// The function is primarily used by the REPL to nicely format execution result.
    pub fn repr(&self) -> String {
        // This is a convenient way to more briefly referrer to the items
        // of the current enum. So, instead of `Value::Def` we can write just `Def`.
        use Value::*;
        match self {
            Def { arg, value } => {
                format!("λ{arg} {}", value.repr())
            }
            // Global IDs are better to be referred in repr by their name.
            BoundId {
                name, global: true, ..
            }
            | Id { name } => name.to_string(),
            // Local bound IDs should be represented by their values.
            // If you repr them by name, the result of `(λa λb a) true` will be
            // represented as `λb a` and it won't be clear what is `a` in this case.
            // However, if you repr them by value, the repr of the result
            // will be `λb true`.
            BoundId { value, .. } => value.repr(),
            Call { target, arg } => {
                let mut tr = target.repr();
                let mut ar = arg.repr();
                // Only `Def` and `Call` may contain spaces.
                // `Def` needs to be wrapped into braces because
                // `(λa a) b` and `λa a b` are different expressions.
                // `Call` does not need to be wrapped when on the left
                // because `(a b) c` and `a b c` is the same.
                if tr.contains('λ') {
                    tr = format!("({tr})");
                }
                if ar.contains(' ') {
                    ar = format!("({ar})");
                }
                format!("{tr} {ar}")
            }
        }
    }

    /// Bind all unbound variables in the Value to the global names.
    ///
    /// We call it as soon as a new function or expression is defined.
    /// We could postpone doing it until the expression is evaluated,
    /// but then if the user has redefined the name, it will redefine it
    /// for all existing expressions.
    ///
    /// This is especially important for modules. For example, stdlib defines
    /// `true` and then `and` that uses `true`. If the user redefines the `true`
    /// name, it will change the behavior of `and`. That's bad.
    ///
    /// Python keeps a separate scope for all modules and binds all definitions
    /// to the scope in which they are defined. This is better in some cases
    /// but might get pretty messy.
    ///
    /// The disadvantage of binding global names as soon as they are defined
    /// is that you cannot use the names that aren't defined yet. In particular,
    /// it makes recursion impossible without using Y-combinator.
    /// Maybe, this is for better. Otherwise, `a = a` would explode.
    pub fn bind_global(&self, global: &GlobalScope) -> Value {
        use Value::*;
        match self {
            Def { arg, value } => Def {
                arg: arg.to_string(),
                value: value.bind_global(global).into(),
            },
            Id { name } => match global.get(name) {
                Some(val) => BoundId {
                    name: name.to_string(),
                    value: val.clone().into(),
                    global: true,
                },
                // You'll see me doing `clone` a lot in this module.
                // Perhaps, I could avoid it with some smart pointer
                // (most likely, `Rc` or `Cow`) but there are a few
                // reasons not to do it:
                //
                // 1. Smart pointers are hard and I'm not good at it.
                // 2. Premature optimization is the root of all evil.
                //    If you want to optimize this code, I encourage
                //    you to write benchmarks first.
                // 3. Mutability is hard an error-prone. If we try
                //    sharing a global Def then accidentally bind
                //    to it some local variable, it might lead to
                //    nasty bugs.
                //
                // So, for a PoC (and RLCI is PoC), keep it simple.
                None => self.clone(),
            },
            BoundId { .. } => self.clone(),
            Call { target, arg } => Call {
                target: target.bind_global(global).into(),
                arg: arg.bind_global(global).into(),
            },
        }
    }

    /// Evaluate the value.
    ///
    /// "Evaluation" is not quite correct word for it, but I wanted to keep
    /// the naming close to the mainstream programming languages.
    /// In lambda calculus, there are no primitive values, only expresssions,
    /// and the expressions can only be converted to other equivalent expressions.
    /// So, there is nothing to evaluate, you can only simplify expressions.
    /// In particular, you can replace a function application with the function
    /// with the argument name replaced by argument value. For example,
    /// `(λa λb a) true` can be simplified into `λb true`.
    /// In lambda calculus, this process is called "[β-reduction]".
    /// This is exactly what this method does.
    ///
    /// [β-reduction]: https://en.wikipedia.org/wiki/Lambda_calculus#Reduction
    pub fn eval(&self) -> anyhow::Result<Value> {
        use Value::*;
        Ok(match self {
            Def { arg: _, value: _ } => self.clone(),
            Id { name } => anyhow::bail!("unbound variable `{name}`"),
            BoundId { name, value, .. } => {
                // Adding context to every failure is very important
                // during the evaluation. This is how we build a traceback
                // that get shown to the user in case of an undefined variable.
                value.eval().context(format!("failure executing {name}"))?
            }
            Call { target, arg } => {
                // This is the star of the show. The main point of the evaluation
                // is to call all functions.
                target.call(arg).context("failure calling a function")?
            }
        })
    }

    /// Call the value with the given argument value.
    ///
    /// Since in lambda calculus everything is a function,
    /// everything can be called and it never fails.
    /// The only way this method can fail is if a variable used is undefined.
    pub fn call(&self, arg_value: &Value) -> anyhow::Result<Value> {
        use Value::*;
        match self {
            Def {
                arg: arg_name,
                value: expr,
            } => {
                let expr = expr.bind_local(arg_name, arg_value);
                expr.eval()
            }
            Id { name } => anyhow::bail!("unbound variable `{name}`"),
            BoundId { value, .. } => value.call(arg_value),
            Call { target, arg } => {
                let value = target.call(arg)?;
                value.call(arg_value)
            }
        }
    }

    /// Recursively bind the given name to the given value.
    ///
    /// It's called `bind_local` because it is called for function application
    /// during evaluation (β-reduction).
    fn bind_local(&self, lname: &str, lvalue: &Value) -> Value {
        use Value::*;
        match self {
            Def { arg, value } => {
                // Do not bind the local variable to functions that will shadow it anyway.
                // This is not just a performance improvement. If we bind a variable
                // that is meant to be rebind later, we may get a wrong repr.
                // For example: `(λa λa a) id`.
                // The repr for the evaluation result of this expression should be `λa a`
                // but without the check below it will be `λa id` which is wrong.
                if arg == lname {
                    self.clone()
                } else {
                    Def {
                        arg: arg.to_string(),
                        value: value.bind_local(lname, lvalue).into(),
                    }
                }
            }
            // Bind the local value to the Id (bound or unbound) if the name matches.
            //
            // We allow rebinding already bound IDs. It allows for shadowing
            // global names with local ones.
            Id { name } | BoundId { name, .. } => {
                if name == lname {
                    BoundId {
                        name: name.to_string(),
                        value: lvalue.clone().into(),
                        global: false,
                    }
                } else {
                    self.clone()
                }
            }
            Call { target, arg } => {
                let target = target.bind_local(lname, lvalue);
                let arg = arg.bind_local(lname, lvalue);
                Call {
                    target: target.into(),
                    arg: arg.into(),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast_nodes::Stmt;
    use crate::parse;
    use rstest::rstest;

    // Test that parsing and `repr`ing an expression gives an expected result.
    //
    // Some people believe that unit-tests must be religiously isolated,
    // so you cannot `parse` here but instead have to manually write and pass AST
    // nodes, to ensure that if something breaks in the parser, it won't affect
    // runtime tests.
    //
    // I believe that the tests must be simple.
    #[rstest]
    #[case::id(r"id", "id")]
    #[case::id(r"\x x", "λx x")]
    #[case::id(r"\a \b a b", "λa λb a b")]
    #[case::id(r"\a \b (\c c) b", "λa λb (λc c) b")]
    #[case::id(r"\a \b a (b a)", "λa λb a (b a)")]
    #[case::id(r"\a \b a b a", "λa λb a b a")]
    #[case::id(r"\a \b (a b) a", "λa λb a b a")]
    fn parse_and_repr(#[case] input: &str, #[case] exp: &str) {
        let module = parse(input).unwrap();
        assert_eq!(module.stmts.len(), 1);
        let stmt = &module.stmts[0];
        match stmt {
            Stmt::Expr { expr } => assert_eq!(Value::from_expr(expr).repr(), exp),
            Stmt::Assign { .. } => panic!("bad statement"),
        }
    }
}
