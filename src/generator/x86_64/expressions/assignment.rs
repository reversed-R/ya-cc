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
        // let relat = &self.right.left;
        // let arithm = &relat.left;
        // let mul = &arithm.left;
        // let unary = &mul.left;
        // let ref_unary = &unary.right;
        //
        // if self.right.rights.is_empty()
        //     && relat.rights.is_empty()
        //     && arithm.rights.is_empty()
        //     && mul.rights.is_empty()
        //     && UnaryOperator::Plus == unary.op
        //     && ref_unary.ops.is_empty()
        // {
        //     if let Primary::Identifier(id) = &ref_unary.right {
        //         return Some(id);
        //     }
        // }

        None
    }
}

impl AssignExpr {
    pub fn assignable_variable(&self) -> Option<&String> {
        // let equal = &self.left;
        // let relat = &equal.left;
        // let arithm = &relat.left;
        // let mul = &arithm.left;
        // let unary = &mul.left;
        // let ref_unary = &unary.right;
        //
        // if equal.rights.is_empty()
        //     && relat.rights.is_empty()
        //     && arithm.rights.is_empty()
        //     && mul.rights.is_empty()
        //     && UnaryOperator::Plus == unary.op
        //     && ref_unary.ops.is_empty()
        // {
        //     if let Primary::Identifier(id) = &ref_unary.right {
        //         return Some(id);
        //     }
        // }

        None
    }
}

impl LocalGenerate for AssignExpr {
    fn generate(&self, env: &mut crate::generator::x86_64::globals::Env) {
        self.right.generate(env);

        for ass in self.lefts.iter().rev() {
            match ass.left.op {
                UnaryOperator::Plus => {
                    println!("pop rdi");

                    if ass.left.right.ops.is_empty() {
                        match &ass.left.right.right {
                            Primary::Identifier(id) => {
                                if let Some(offset) = env.offset(id) {
                                    ass.op.generate(offset);
                                } else {
                                    panic!("Local Variable Not Found");
                                }
                            }
                            _ => {
                                panic!("Invalid Left Value");
                            }
                        }
                    }

                    println!("push rdi");
                }
                UnaryOperator::Minus => {
                    panic!("Invalid Left Value");
                }
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
