use crate::interpreter::value::Value;

trait Scope {
    fn get(&self) -> Value;
}

struct GlobalScope {}

impl Scope for GlobalScope {
    fn get(&self) -> Value {
        todo!()
    }
}
