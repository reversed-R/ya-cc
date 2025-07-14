use crate::{
    generator::x86_64::LocalGenerate, parser::symbols::expressions::assignment::AssignExpr,
};

impl LocalGenerate for AssignExpr {
    fn generate(&self, locals: &std::collections::HashMap<String, usize>) {
        // TODO:
        // local variables assignment
        self.left.generate(locals);
    }
}
