use crate::{
    generator::x86_64::globals::LocalGenerate,
    parser::symbols::expressions::{
        assignment::{AssignExpr, AssignExprNode, AssignOperator},
        primary::Primary,
        unary::UnaryOperator,
    },
};

impl AssignExprNode {
    fn assignable_variable(&self) -> Option<&String> {
        let relat = &self.right.left;
        let arithm = &relat.left;
        let mul = &arithm.left;
        let unary = &mul.left;

        if self.right.rights.is_empty()
            && relat.rights.is_empty()
            && arithm.rights.is_empty()
            && mul.rights.is_empty()
            && UnaryOperator::Plus == unary.op
        {
            if let Primary::Identifier(id) = &unary.right {
                return Some(id);
            }
        }

        None
    }
}

impl AssignExpr {
    pub fn assignable_variable(&self) -> Option<&String> {
        let equal = &self.left;
        let relat = &equal.left;
        let arithm = &relat.left;
        let mul = &arithm.left;
        let unary = &mul.left;

        if equal.rights.is_empty()
            && relat.rights.is_empty()
            && arithm.rights.is_empty()
            && mul.rights.is_empty()
            && UnaryOperator::Plus == unary.op
        {
            if let Primary::Identifier(id) = &unary.right {
                return Some(id);
            }
        }

        None
    }
}

impl LocalGenerate for AssignExpr {
    fn generate(&self, env: &mut crate::generator::x86_64::globals::Env) {
        if self.rights.is_empty() {
            self.left.generate(env);
        } else {
            let mut last_op: &AssignOperator = &AssignOperator::Assign;

            for (i, ass) in self.rights.iter().rev().enumerate() {
                if i == 0 {
                    ass.right.generate(env);
                    last_op = &ass.op;
                } else if let Some(id) = ass.assignable_variable() {
                    ass.right.generate(env);

                    println!("pop rdi");

                    if let Some(offset) = env.offset(id) {
                        last_op.generate(offset);
                    } else {
                        panic!("Local Variable Not Found");
                    }

                    println!("push rdi");

                    last_op = &ass.op;
                } else {
                    panic!("Invalid Left Value");
                }
            }

            if let Some(id) = self.assignable_variable() {
                println!("pop rdi");

                if let Some(offset) = env.offset(id) {
                    last_op.generate(offset);
                } else {
                    panic!("Local Variable Not Found");
                }

                println!("push rdi");
            } else {
                panic!("Invalid Left Value");
            }
        }
    }
}

impl AssignOperator {
    fn generate(&self, offset: usize) {
        match self {
            Self::Assign => {
                println!("mov [rbp - {offset}], rdi");
            }
        }
    }
}
