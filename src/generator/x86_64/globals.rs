use std::collections::HashMap;

use crate::{
    generator::x86_64::LocalGenerate,
    parser::symbols::{expressions::primary::Primary, globals::FnDec, statements::Stmt},
};

impl FnDec {
    pub fn generate(&self) {
        let locals = self.list_local_variables();

        println!("{}:", self.name);

        for stmt in &self.stmts {
            stmt.generate(&locals);
        }

        println!("ret");
    }

    fn list_local_variables(&self) -> HashMap<String, usize> {
        let mut locals: HashMap<String, usize> = HashMap::new();
        const SIZE_OF_VARIABLE: usize = 4;

        for stmt in &self.stmts {
            if let Stmt::Expr(expr) = stmt {
                let ass = &expr.0;

                let equal = &ass.left;
                let relat = &equal.left;
                let arithm = &relat.left;
                let mul = &arithm.left;

                if equal.rights.is_empty()
                    && relat.rights.is_empty()
                    && arithm.rights.is_empty()
                    && mul.rights.is_empty()
                {
                    if let Primary::Identifier(id) = &mul.left.right {
                        if !locals.contains_key(id) {
                            locals.insert(id.clone(), locals.len() * SIZE_OF_VARIABLE);
                        }
                    }
                } else {
                    panic!("Invalid Left Value");
                }
            }
        }

        locals
    }
}
