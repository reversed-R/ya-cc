use crate::{
    generator::x86_64::Generate, parser::symbols::expressions::relational::RelationalExpr,
};

impl Generate for RelationalExpr {
    fn generate(&self) {}
}
