use crate::{generator::x86_64::Generate, parser::symbols::expressions::assignment::AssignExpr};

impl Generate for AssignExpr {
    fn generate(&self) {
        // TODO:
        // local variables assignment
        if let Some(ass) = self.nodes.first() {
            ass.right.generate();
        }
    }
}
