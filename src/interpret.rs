use crate::{output::GraphOutput, parse::ast::Program};

struct Interpreter {
    ast: Program,
    output: GraphOutput,
}
impl Interpreter {
    fn new(ast: Program) -> Self {
        return Self {
            ast,
            output: GraphOutput::default(),
        };
    }
    fn interpret(&self) {}
}
